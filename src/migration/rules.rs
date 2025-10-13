// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

mod distributed_cache_services;
mod jolt_transform_json;
mod jolt_transform_record;

pub use distributed_cache_services::DistributedCacheServicesMigration;
pub use jolt_transform_json::JoltTransformJsonMigration;
pub use jolt_transform_record::JoltTransformRecordMigration;

use serde_json::Value;

/// Represents a migration rule that can be applied to processors and controller services.
pub trait MigrationRule {
    /// Check if this rule applies to the given component.
    fn applies(&self, component: &Value) -> bool;

    /// Apply the migration to the component, returning `true` if changes were made.
    fn apply(&self, component: &mut Value) -> bool;

    /// Get a description of what this rule does.
    fn description(&self) -> String;
}
