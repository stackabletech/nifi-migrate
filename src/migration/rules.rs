// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

mod jolt_transform_json;
mod jolt_transform_record;

pub use jolt_transform_json::JoltTransformJsonMigration;
pub use jolt_transform_record::JoltTransformRecordMigration;

use serde_json::Value;

/// Represents a migration rule that can be applied to processors.
pub trait MigrationRule {
    /// Check if this rule applies to the given processor.
    fn applies(&self, processor: &Value) -> bool;

    /// Apply the migration to the processor, returning `true` if changes were made.
    fn apply(&self, processor: &mut Value) -> bool;

    /// Get a description of what this rule does.
    fn description(&self) -> String;
}
