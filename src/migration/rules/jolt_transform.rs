// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

use super::MigrationRule;
use serde_json::Value;

/// Migration rule for JoltTransformJSON processor.
///
/// Migrates `org.apache.nifi.processors.standard.JoltTransformJSON` to
/// `org.apache.nifi.processors.jolt.JoltTransformJSON` and updates the
/// bundle from `nifi-standard-nar` to `nifi-jolt-nar`.
///
/// Reference: <https://issues.apache.org/jira/browse/NIFI-12554>
pub struct JoltTransformMigration;

impl MigrationRule for JoltTransformMigration {
    fn applies(&self, processor: &Value) -> bool {
        processor
            .get("type")
            .and_then(|t| t.as_str())
            .map(|t| t == "org.apache.nifi.processors.standard.JoltTransformJSON")
            .unwrap_or(false)
    }

    fn apply(&self, processor: &mut Value) -> bool {
        let mut changed = false;

        // Update the type field
        if let Some(type_field) = processor.get_mut("type") {
            if type_field.as_str() == Some("org.apache.nifi.processors.standard.JoltTransformJSON")
            {
                *type_field =
                    Value::String("org.apache.nifi.processors.jolt.JoltTransformJSON".to_owned());
                changed = true;
            }
        }

        // Update the bundle artifact field
        if let Some(bundle) = processor.get_mut("bundle") {
            if let Some(artifact) = bundle.get_mut("artifact") {
                if artifact.as_str() == Some("nifi-standard-nar") {
                    *artifact = Value::String("nifi-jolt-nar".to_owned());
                    changed = true;
                }
            }
        }

        changed
    }

    fn description(&self) -> String {
        "Migrate JoltTransformJSON from standard to jolt bundle".to_owned()
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
    fn test_full_real_world_processor() {
        let mut processor = json!({
            "identifier": "347aaa3b-2b3a-30c5-932d-58a109f9478f",
            "instanceIdentifier": "e70d7d94-e92a-3472-b1c7-338e291af5e9",
            "name": "JoltTransformJSON",
            "comments": "",
            "position": {
                "x": 3368.0,
                "y": 200.0
            },
            "type": "org.apache.nifi.processors.standard.JoltTransformJSON",
            "bundle": {
                "group": "org.apache.nifi",
                "artifact": "nifi-standard-nar",
                "version": "1.18.0"
            },
            "properties": {},
            "propertyDescriptors": {
                "jolt-spec": {
                    "name": "jolt-spec",
                    "displayName": "jolt-spec",
                    "identifiesControllerService": false,
                    "sensitive": true,
                    "dynamic": false
                },
                "jolt-transform": {
                    "name": "jolt-transform",
                    "displayName": "jolt-transform",
                    "identifiesControllerService": false,
                    "sensitive": true,
                    "dynamic": false
                },
                "pretty_print": {
                    "name": "pretty_print",
                    "displayName": "pretty_print",
                    "identifiesControllerService": false,
                    "sensitive": true,
                    "dynamic": false
                },
                "Transform Cache Size": {
                    "name": "Transform Cache Size",
                    "displayName": "Transform Cache Size",
                    "identifiesControllerService": false,
                    "sensitive": true,
                    "dynamic": false
                }
            },
            "style": {},
            "schedulingPeriod": "0 sec",
            "schedulingStrategy": "TIMER_DRIVEN",
            "executionNode": "ALL",
            "penaltyDuration": "30 sec",
            "yieldDuration": "1 sec",
            "bulletinLevel": "WARN",
            "runDurationMillis": 0,
            "concurrentlySchedulableTaskCount": 1,
            "autoTerminatedRelationships": ["failure"],
            "scheduledState": "ENABLED",
            "retryCount": 10,
            "retriedRelationships": [],
            "backoffMechanism": "PENALIZE_FLOWFILE",
            "maxBackoffPeriod": "10 mins",
            "componentType": "PROCESSOR",
            "groupIdentifier": "5e61ca9d-43f8-3176-b706-2404009bcc5b"
        });

        let rule = JoltTransformMigration;

        // Verify it applies to this processor
        assert!(
            rule.applies(&processor),
            "Rule should apply to JoltTransformJSON processor"
        );

        // Apply the migration
        assert!(rule.apply(&mut processor), "Migration should make changes");

        // Verify the type was changed
        assert_eq!(
            processor.get("type").and_then(|v| v.as_str()),
            Some("org.apache.nifi.processors.jolt.JoltTransformJSON"),
            "Processor type should be updated"
        );

        // Verify the bundle artifact was changed
        assert_eq!(
            processor
                .get("bundle")
                .and_then(|b| b.get("artifact"))
                .and_then(|v| v.as_str()),
            Some("nifi-jolt-nar"),
            "Bundle artifact should be updated to nifi-jolt-nar"
        );

        // Verify other bundle fields remain unchanged
        assert_eq!(
            processor
                .get("bundle")
                .and_then(|b| b.get("group"))
                .and_then(|v| v.as_str()),
            Some("org.apache.nifi"),
            "Bundle group should remain unchanged"
        );
        assert_eq!(
            processor
                .get("bundle")
                .and_then(|b| b.get("version"))
                .and_then(|v| v.as_str()),
            Some("1.18.0"),
            "Bundle version should remain unchanged"
        );

        // Verify other fields remain unchanged
        assert_eq!(
            processor.get("identifier").and_then(|v| v.as_str()),
            Some("347aaa3b-2b3a-30c5-932d-58a109f9478f"),
            "Identifier should remain unchanged"
        );
        assert_eq!(
            processor.get("name").and_then(|v| v.as_str()),
            Some("JoltTransformJSON"),
            "Name should remain unchanged"
        );
    }
}
