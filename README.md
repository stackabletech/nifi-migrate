<!--
SPDX-FileCopyrightText: 2025 Stackable GmbH
SPDX-License-Identifier: Apache-2.0
-->

# nifi-migrate

A Rust CLI tool for migrating Apache NiFi flow.json files between versions.

## Features

- **Non-destructive**: Reads from input file and writes to a separate output file
- **Recursive**: Processes nested process groups automatically
- **Extensible**: Easy to add new migration rules via the trait-based system
- **Safe**: Distinguishes between processors and controller services to avoid incorrect migrations

## Supported Migrations

### JoltTransformJSON Processor

- **Type**: `org.apache.nifi.processors.standard.JoltTransformJSON` → `org.apache.nifi.processors.jolt.JoltTransformJSON`
- **Bundle artifact**: `nifi-standard-nar` → `nifi-jolt-nar`
- **Reason**: In NiFi 2.x, Jolt processors were moved to a separate bundle
- **Reference**: [NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554)

### JoltTransformRecord Processor

- **Type**: `org.apache.nifi.processors.jolt.record.JoltTransformRecord` → `org.apache.nifi.processors.jolt.JoltTransformRecord`
- **Bundle artifact**: `nifi-jolt-record-nar` → `nifi-jolt-nar`
- **Reason**: Consolidated into the main jolt bundle
- **Reference**: [NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554)

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/nifi-migrate`

## Usage

Basic usage:

```bash
nifi-migrate --input flow.json --output flow-migrated.json
```

Or using the cargo alias:

```bash
cargo nifi-migrate --input flow.json --output flow-migrated.json
```

With pretty-printed JSON output:

```bash
nifi-migrate --input flow.json --output flow-migrated.json --pretty
```

### Options

- `-i, --input <PATH>`: Input flow.json file (required)
- `-o, --output <PATH>`: Output flow.json file (required)
- `-p, --pretty`: Pretty-print the output JSON (optional, default is compact)
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Important Notes

### JSON Formatting

⚠️ **The tool will reformat your JSON file.** By default, output is compact (single line). Use `--pretty` flag for human-readable formatting with indentation.

The order of JSON keys may also change as the file is parsed and reserialized. While this doesn't affect NiFi's ability to read the file, it may make git diffs larger.

## Adding New Migration Rules

To add a new migration rule:

1. Create a new file in `src/rules/` (e.g., `my_rule.rs`)
1. Implement the `MigrationRule` trait:

```rust
use super::MigrationRule;
use serde_json::Value;

pub struct MyMigrationRule;

impl MigrationRule for MyMigrationRule {
    fn applies(&self, processor: &Value) -> bool {
        // Check if this rule applies
        processor.get("type")
            .and_then(|t| t.as_str())
            .map(|t| t == "org.apache.nifi.processors.old.Processor")
            .unwrap_or(false)
    }

    fn apply(&self, processor: &mut Value) -> bool {
        // Apply the migration
        if let Some(type_field) = processor.get_mut("type") {
            *type_field = Value::String("org.apache.nifi.processors.new.Processor".to_string());
            return true;
        }
        false
    }

    fn description(&self) -> String {
        "Migrate Processor from old to new package".to_string()
    }
}
```

1. Add your rule to `src/rules/mod.rs`:

```rust
mod my_rule;
pub use my_rule::MyMigrationRule;
```

1. Register it in `Migrator::new()` in `src/lib.rs`:

```rust
pub fn new() -> Self {
    Self {
        rules: vec![
            Box::new(JoltTransformMigration),
            Box::new(MyMigrationRule),  // Add your rule here
        ],
    }
}
```

1. Add tests to verify your rule works correctly

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please ensure:

- No new clippy lints (`cargo clippy --all-targets -- -D warnings`)
- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- REUSE compliance (`reuse lint`)
- Add tests for new migration rules

### Running All Checks

You can run all checks at once using:

```bash
just all
```

Or run pre-commit hooks:

```bash
just pre-commit
```
