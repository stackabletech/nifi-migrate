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

## Releases

### How to Create a Release

1. **Update version numbers** in `Cargo.toml`:

   ```toml
   [package]
   version = "0.2.0"  # Update to new version
   ```

2. **Update CHANGELOG.md**:
   - Move items from `[Unreleased]` section to a new version section
   - Add the release date
   - Update the comparison links at the bottom
   - Example:

     ```markdown
     ## [0.2.0] - 2025-10-14

     ### Added
     - New migration rule for XYZ

     [Unreleased]: https://github.com/stackabletech/nifi-migrate/compare/rel/nifi-migrate-0.2.0...HEAD
     [0.2.0]: https://github.com/stackabletech/nifi-migrate/releases/tag/rel/nifi-migrate-0.2.0
     ```

3. **Create a pull request** with these changes:
   - Title: `Prepare release 0.2.0`
   - Ensure all CI checks pass
   - Get approval and merge to `main`

4. **Create and push the release tag**:

   ```bash
   git checkout main
   git pull origin main
   git tag rel/nifi-migrate-0.2.0
   git push origin rel/nifi-migrate-0.2.0
   ```

5. **Automated release process**:
   - GitHub Actions will automatically:
     - Create a draft release
     - Build binaries for 4 platforms:
       - `nifi-migrate-aarch64-unknown-linux-gnu` (Linux ARM64)
       - `nifi-migrate-x86_64-unknown-linux-gnu` (Linux x86_64)
       - `nifi-migrate-aarch64-apple-darwin` (macOS ARM64)
       - `nifi-migrate-x86_64-pc-windows-msvc.exe` (Windows x86_64)
     - Upload all binaries to the release
     - Automatically publish the release (no manual step needed)

6. **Verify the release**:
   - Check <https://github.com/stackabletech/nifi-migrate/releases>
   - Download and test binaries on different platforms
   - Update documentation if needed

### Release Checklist

- [ ] Version bumped in `Cargo.toml`
- [ ] `CHANGELOG.md` updated with new version and release date
- [ ] All migration rules documented in README's "Supported Migrations" section
- [ ] All tests passing locally (`just all`)
- [ ] PR created, reviewed, and merged
- [ ] Tag created with format `rel/nifi-migrate-X.Y.Z`
- [ ] Tag pushed to origin
- [ ] GitHub release automatically published with all 4 binaries

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
