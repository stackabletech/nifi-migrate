// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

use super::MigrationRule;
use serde_json::Value;

/// Migration rule for JoltTransformRecord processor.
///
/// Migrates `org.apache.nifi.processors.jolt.record.JoltTransformRecord` to
/// `org.apache.nifi.processors.jolt.JoltTransformRecord` and updates the
/// bundle from `nifi-jolt-record-nar` to `nifi-jolt-nar`.
///
/// Reference: <https://issues.apache.org/jira/browse/NIFI-12554>
pub struct JoltTransformRecordMigration;

impl MigrationRule for JoltTransformRecordMigration {
    fn applies(&self, processor: &Value) -> bool {
        processor
            .get("type")
            .and_then(|t| t.as_str())
            .map(|t| t == "org.apache.nifi.processors.jolt.record.JoltTransformRecord")
            .unwrap_or(false)
    }

    fn apply(&self, processor: &mut Value) -> bool {
        let mut changed = false;

        // Update the type field
        if let Some(type_field) = processor.get_mut("type") {
            if type_field.as_str()
                == Some("org.apache.nifi.processors.jolt.record.JoltTransformRecord")
            {
                *type_field =
                    Value::String("org.apache.nifi.processors.jolt.JoltTransformRecord".to_owned());
                changed = true;
            }
        }

        // Update the bundle artifact field
        if let Some(bundle) = processor.get_mut("bundle") {
            if let Some(artifact) = bundle.get_mut("artifact") {
                if artifact.as_str() == Some("nifi-jolt-record-nar") {
                    *artifact = Value::String("nifi-jolt-nar".to_owned());
                    changed = true;
                }
            }
        }

        changed
    }

    fn description(&self) -> String {
        "Migrate JoltTransformRecord from jolt-record to jolt bundle".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_jolt_transform_record_migration() {
        let mut processor = json!({
            "identifier": "test-id-456",
            "name": "JoltTransformRecord",
            "type": "org.apache.nifi.processors.jolt.record.JoltTransformRecord",
            "bundle": {
                "artifact": "nifi-jolt-record-nar",
                "group": "org.apache.nifi",
                "version": "1.27.0"
            }
        });

        let rule = JoltTransformRecordMigration;
        assert!(rule.applies(&processor));
        assert!(rule.apply(&mut processor));

        assert_eq!(
            processor.get("type").and_then(|v| v.as_str()),
            Some("org.apache.nifi.processors.jolt.JoltTransformRecord")
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
    fn test_does_not_apply_to_other_processors() {
        let processor = json!({
            "identifier": "other-proc",
            "name": "SomeOtherProcessor",
            "type": "org.apache.nifi.processors.standard.LogAttribute",
            "bundle": {
                "artifact": "nifi-standard-nar"
            }
        });

        let rule = JoltTransformRecordMigration;
        assert!(!rule.applies(&processor));
    }
}
