// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

mod rules;

use anyhow::{Context, Result};
use rules::{JoltTransformMigration, JoltTransformRecordMigration, MigrationRule};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Represents a change that would be made during migration
#[derive(Debug, Clone)]
pub struct MigrationChange {
    pub processor_id: String,
    pub processor_name: String,
    pub rule_description: String,
}

/// Main migration engine
pub struct Migrator {
    rules: Vec<Box<dyn MigrationRule>>,
}

impl Migrator {
    /// Create a new migrator with default rules
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(JoltTransformMigration),
                Box::new(JoltTransformRecordMigration),
            ],
        }
    }

    /// Migrate a flow JSON file
    pub fn migrate_file(
        &self,
        input_path: &Path,
        output_path: &Path,
        pretty: bool,
    ) -> Result<Vec<MigrationChange>> {
        // Validate input exists
        if !input_path.exists() {
            anyhow::bail!("Input file does not exist: {}", input_path.display());
        }

        // Warn if input and output are the same
        let canonical_input = input_path
            .canonicalize()
            .with_context(|| format!("Failed to resolve input path: {}", input_path.display()))?;

        if let Ok(canonical_output) = output_path.canonicalize() {
            if canonical_input == canonical_output {
                anyhow::bail!(
                    "Input and output paths are the same. This would overwrite the original file."
                );
            }
        }

        // Validate output directory exists
        if let Some(parent) = output_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                anyhow::bail!("Output directory does not exist: {}", parent.display());
            }
        }

        // It's not perfect reading it all in memory, but I decided it's fine for now.
        // I tried it on a reasonably large file and it was fine.
        // We can switch to streaming if it's ever needed.
        let content = fs::read_to_string(input_path)
            .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;

        let mut flow: Value = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from: {}", input_path.display()))?;

        let changes = self.migrate_flow(&mut flow)?;

        if !changes.is_empty() {
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

    /// Migrate a flow JSON value in-place
    fn migrate_flow(&self, flow: &mut Value) -> Result<Vec<MigrationChange>> {
        let mut changes = Vec::new();
        self.process_value(flow, &mut changes);
        Ok(changes)
    }

    /// Recursively process a JSON value looking for processors
    fn process_value(&self, value: &mut Value, changes: &mut Vec<MigrationChange>) {
        self.process_value_with_context(value, None, changes);
    }

    /// Recursively process a JSON value with parent key context.
    /// The NiFi JSON is not very deep so recursive should not cause any issues here.
    fn process_value_with_context(
        &self,
        value: &mut Value,
        parent_key: Option<&str>,
        changes: &mut Vec<MigrationChange>,
    ) {
        match value {
            Value::Object(map) => {
                // Check if this object is a processor (but not a controller service)
                // Controller services have the same structure as processors (type + bundle)
                // but appear under "controllerServices" key instead of "processors" key
                // This entire matching thing (as well as the migration rules) can be made smarter
                // as needed. For now, we only have two rules and both are for processors so it's
                // fine as is.
                let is_processor = map.contains_key("type")
                    && map.contains_key("bundle")
                    && parent_key != Some("controllerServices");

                if is_processor {
                    self.process_processor(value, changes);
                }

                // Recursively process all nested values
                // Need to re-borrow to avoid double mutable borrow
                if let Value::Object(map) = value {
                    for (key, val) in map.iter_mut() {
                        self.process_value_with_context(val, Some(key), changes);
                    }
                }
            }
            Value::Array(arr) => {
                for item in arr.iter_mut() {
                    self.process_value_with_context(item, parent_key, changes);
                }
            }
            _ => {}
        }
    }

    /// Process a single processor object
    fn process_processor(&self, processor: &mut Value, changes: &mut Vec<MigrationChange>) {
        for rule in &self.rules {
            if rule.applies(processor) && rule.apply(processor) {
                let processor_id = processor
                    .get("identifier")
                    .or_else(|| processor.get("id"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let processor_name = processor
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unnamed")
                    .to_string();

                changes.push(MigrationChange {
                    processor_id,
                    processor_name,
                    rule_description: rule.description(),
                });
            }
        }
    }
}

impl Default for Migrator {
    fn default() -> Self {
        Self::new()
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

        let rule = JoltTransformMigration;
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

        let migrator = Migrator::new();
        let changes = migrator.migrate_flow(&mut flow).unwrap();

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

        let migrator = Migrator::new();
        let changes = migrator.migrate_flow(&mut flow).unwrap();

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
}
