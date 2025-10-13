<!--
SPDX-FileCopyrightText: 2025 Stackable GmbH
SPDX-License-Identifier: Apache-2.0
-->

# nifi-migrate

A Rust CLI tool for migrating Apache NiFi flow.json files between versions.

## Supported Migrations

### JoltTransformJSON Processor

- **Type**: `org.apache.nifi.processors.standard.JoltTransformJSON` → `org.apache.nifi.processors.jolt.JoltTransformJSON`
- **Bundle artifact**: `nifi-standard-nar` → `nifi-jolt-nar`
- **Reason**: In NiFi 2.x, Jolt processors were moved to a separate bundle and properties were renamed
- **Reference**: [NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554)

### JoltTransformRecord Processor

- **Type**: `org.apache.nifi.processors.jolt.record.JoltTransformRecord` → `org.apache.nifi.processors.jolt.JoltTransformRecord`
- **Bundle artifact**: `nifi-jolt-record-nar` → `nifi-jolt-nar`
- **Reason**: In NiFi 2.x, Jolt processors were moved to a separate bundle and properties were renamed
- **Reference**: [NIFI-12554](https://issues.apache.org/jira/browse/NIFI-12554)

### Distributed Cache Controller Services

All Distributed Cache services have been renamed to remove the "Distributed" prefix for clarity in NiFi 2.x.

- **DistributedMapCacheClientService** → **MapCacheClientService**
- **DistributedSetCacheClientService** → **SetCacheClientService**
- **DistributedMapCacheServer** → **MapCacheServer**
- **DistributedSetCacheServer** → **SetCacheServer**
- **Reference**: [NIFI-13596](https://issues.apache.org/jira/browse/NIFI-13596)

## Build

```bash
cargo build --release
```

The binary will be available at `target/release/nifi-migrate`

## Usage

Basic usage:

```bash
nifi-migrate flow.json flow-migrated.json
```

Or using the cargo alias:

```bash
cargo nifi-migrate flow.json flow-migrated.json
```

With pretty-printed JSON output:

```bash
nifi-migrate flow.json flow-migrated.json --pretty
```

### Options

Call `nifi-migrate --help` to see all its options.

## Important Notes

### JSON Formatting

> [!WARNING]
> The output file will be reformatted. By default, output is compact (single line). Use `--pretty` flag for human-readable formatting with indentation.

The order of JSON keys in the output may also change as the file is parsed and reserialized. While this doesn't affect NiFi's ability to read the file, it may make git diffs larger.

The input file is never modified.

## Adding New Migration Rules

To add a new migration rule:

1. look at one of the existing ones in `src/rules` and follow the pattern (basically: Implement the `MigrationRule` trait).
2. Add your rule to `src/rules/rules.rs`:

    ```rust
    mod my_rule;
    pub use my_rule::MyMigrationRule;
    ```

3. Register it in `Migrator::new()` in `src/migration.rs`:

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

And run pre-commit hooks:

```bash
just pre-commit
```
