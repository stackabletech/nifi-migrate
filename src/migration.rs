// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

mod rules;

use anyhow::{Context, Result};
use rules::{
    DistributedCacheServicesMigration, JoltTransformJsonMigration, JoltTransformRecordMigration,
    MigrationRule,
};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Represents a change that would be made during migration.
#[derive(Debug, Clone)]
pub struct MigrationChange {
    pub processor_id: String,
    pub processor_name: String,
    pub rule_description: String,
}

/// Main migration engine.
pub struct Migrator {
    rules: Vec<Box<dyn MigrationRule>>,
}

impl Default for Migrator {
    fn default() -> Self {
        Self {
            rules: vec![
                // Processor migrations
                Box::new(JoltTransformJsonMigration),
                Box::new(JoltTransformRecordMigration),
                // Controller service migrations
                Box::new(DistributedCacheServicesMigration),
            ],
        }
    }
}

impl Migrator {
    /// Migrate a flow JSON file.
    pub fn migrate_file(
        &self,
        input_path: &Path,
        output_path: &Path,
        pretty: bool,
        format_only: bool,
    ) -> Result<Vec<MigrationChange>> {
        // Validate input exists
        if !input_path.exists() {
            anyhow::bail!("Input file does not exist: {}", input_path.display());
        }

        // Warn if input and output are the same
        let canonical_input = input_path
            .canonicalize()
            .with_context(|| format!("Failed to resolve input path: {}", input_path.display()))?;

        if let Ok(canonical_output) = output_path.canonicalize()
            && canonical_input == canonical_output
        {
            anyhow::bail!(
                "Input and output paths are the same. This would overwrite the original file."
            );
        }

        // Validate output directory exists
        if let Some(parent) = output_path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
        {
            anyhow::bail!("Output directory does not exist: {}", parent.display());
        }

        let file = fs::File::open(input_path)
            .with_context(|| format!("Failed to open input file: {}", input_path.display()))?;
        let reader = std::io::BufReader::new(file);

        let mut flow: Value = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse JSON from: {}", input_path.display()))?;

        let changes = if format_only {
            Vec::new()
        } else {
            self.migrate_flow(&mut flow)
        };

        // In format-only mode, always write output even if no migrations
        // In normal mode, only write if changes were made
        if format_only || !changes.is_empty() {
            let output = if pretty {
                serde_json::to_string_pretty(&flow)
            } else {
                serde_json::to_string(&flow)
            }
            .context("Failed to serialize output JSON")?;

            fs::write(output_path, output).with_context(|| {
                format!("Failed to write output file: {}", output_path.display())
            })?;
        }

        Ok(changes)
    }

    /// Migrate a flow JSON value in-place.
    fn migrate_flow(&self, flow: &mut Value) -> Vec<MigrationChange> {
        let mut changes = Vec::new();
        self.process_value_with_context(flow, None, &mut changes);
        changes
    }

    /// Recursively process a JSON value with parent key context.
    /// The NiFi JSON is not very deep so recursive should not cause any issues here.
    fn process_value_with_context(
        &self,
        value: &mut Value,
        _parent_key: Option<&str>,
        changes: &mut Vec<MigrationChange>,
    ) {
        match value {
            Value::Object(map) => {
                // Check if this object is a processor or controller service.
                // Both have the same structure (type + bundle fields).
                let has_type_and_bundle = map.contains_key("type") && map.contains_key("bundle");

                if has_type_and_bundle {
                    // Apply migration rules (works for both processors and controller services)
                    self.process_component(value, changes);
                }

                // Recursively process all nested values.
                // Need to re-borrow to avoid double mutable borrow.
                if let Value::Object(map) = value {
                    for (key, val) in map.iter_mut() {
                        self.process_value_with_context(val, Some(key), changes);
                    }
                }
            }
            Value::Array(arr) => {
                for item in arr.iter_mut() {
                    self.process_value_with_context(item, _parent_key, changes);
                }
            }
            _ => {}
        }
    }

    /// Process a single component (processor or controller service).
    fn process_component(&self, component: &mut Value, changes: &mut Vec<MigrationChange>) {
        for rule in &self.rules {
            if rule.applies(component) && rule.apply(component) {
                let component_id = component
                    .get("identifier")
                    .or_else(|| component.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_owned();

                let component_name = component
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unnamed")
                    .to_owned();

                changes.push(MigrationChange {
                    processor_id: component_id,
                    processor_name: component_name,
                    rule_description: rule.description(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_jolt_transform_migration() {
        let mut processor = json!({
            "identifier": "test-id-123",
            "name": "JoltTransform",
            "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
            "bundle": {
                "artifact": "nifi-standard-nar",
                "group": "org.apache.nifi",
                "version": "1.25.0"
            }
        });

        let rule = JoltTransformJsonMigration;
        assert!(rule.applies(&processor));
        assert!(rule.apply(&mut processor));

        assert_eq!(
            processor.get("type").and_then(|v| v.as_str()),
            Some("org.apache.nifi.processors.jolt.JoltTransformJSON")
        );
        assert_eq!(
            processor
                .get("bundle")
                .and_then(|b| b.get("artifact"))
                .and_then(|v| v.as_str()),
            Some("nifi-jolt-nar")
        );
    }

    #[test]
    fn test_nested_process_groups() {
        let mut flow = json!({
            "processGroups": [
                {
                    "processors": [
                        {
                            "identifier": "proc-1",
                            "name": "Jolt1",
                            "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
                            "bundle": {
                                "artifact": "nifi-standard-nar"
                            }
                        }
                    ],
                    "processGroups": [
                        {
                            "processors": [
                                {
                                    "identifier": "proc-2",
                                    "name": "Jolt2",
                                    "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
                                    "bundle": {
                                        "artifact": "nifi-standard-nar"
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        });

        let migrator = Migrator::default();
        let changes = migrator.migrate_flow(&mut flow);

        assert_eq!(changes.len(), 2);
        assert!(changes.iter().any(|c| c.processor_id == "proc-1"));
        assert!(changes.iter().any(|c| c.processor_id == "proc-2"));
    }

    #[test]
    fn test_only_migrates_jolt_processors() {
        let mut flow = json!({
            "processors": [
                {
                    "identifier": "other-proc",
                    "name": "SomeOtherProcessor",
                    "type": "org.apache.nifi.processors.standard.LogAttribute",
                    "bundle": {
                        "artifact": "nifi-standard-nar"
                    }
                },
                {
                    "identifier": "jolt-proc",
                    "name": "JoltProcessor",
                    "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
                    "bundle": {
                        "artifact": "nifi-standard-nar"
                    }
                }
            ]
        });

        let migrator = Migrator::default();
        let changes = migrator.migrate_flow(&mut flow);

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].processor_id, "jolt-proc");

        // Verify other processor unchanged
        assert_eq!(
            flow["processors"][0]["bundle"]["artifact"],
            "nifi-standard-nar"
        );
        // Verify jolt processor changed
        assert_eq!(flow["processors"][1]["bundle"]["artifact"], "nifi-jolt-nar");
    }

    #[test]
    fn test_controller_service_migration() {
        let mut flow = json!({
            "flowContents": {
                "controllerServices": [
                    {
                        "identifier": "service-1",
                        "name": "MapCacheClient",
                        "type": "org.apache.nifi.distributed.cache.client.DistributedMapCacheClientService",
                        "bundle": {
                            "artifact": "nifi-distributed-cache-services-nar"
                        }
                    }
                ]
            }
        });

        let migrator = Migrator::default();
        let changes = migrator.migrate_flow(&mut flow);

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].processor_id, "service-1");
        assert_eq!(changes[0].processor_name, "MapCacheClient");

        // Verify the controller service was migrated
        assert_eq!(
            flow["flowContents"]["controllerServices"][0]["type"],
            "org.apache.nifi.distributed.cache.client.MapCacheClientService"
        );
    }

    #[test]
    fn test_mixed_processors_and_controller_services() {
        let mut flow = json!({
            "processors": [
                {
                    "identifier": "proc-1",
                    "name": "JoltProc",
                    "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
                    "bundle": {
                        "artifact": "nifi-standard-nar"
                    }
                }
            ],
            "controllerServices": [
                {
                    "identifier": "service-1",
                    "name": "MapCache",
                    "type": "org.apache.nifi.distributed.cache.client.DistributedMapCacheClientService",
                    "bundle": {
                        "artifact": "nifi-distributed-cache-services-nar"
                    }
                },
                {
                    "identifier": "service-2",
                    "name": "SetCache",
                    "type": "org.apache.nifi.distributed.cache.client.DistributedSetCacheClientService",
                    "bundle": {
                        "artifact": "nifi-distributed-cache-services-nar"
                    }
                }
            ]
        });

        let migrator = Migrator::default();
        let changes = migrator.migrate_flow(&mut flow);

        // Should migrate 1 processor + 2 controller services = 3 total
        assert_eq!(changes.len(), 3);

        // Verify processor migration
        assert_eq!(
            flow["processors"][0]["type"],
            "org.apache.nifi.processors.jolt.JoltTransformJSON"
        );

        // Verify controller service migrations
        assert_eq!(
            flow["controllerServices"][0]["type"],
            "org.apache.nifi.distributed.cache.client.MapCacheClientService"
        );
        assert_eq!(
            flow["controllerServices"][1]["type"],
            "org.apache.nifi.distributed.cache.client.SetCacheClientService"
        );
    }
}
