<!--
SPDX-FileCopyrightText: 2025 Stackable GmbH
SPDX-License-Identifier: Apache-2.0
-->

# Claude Code Instructions

Always read the README.md file to understand what this project is about.

When making changes to this project, always run the following checks in order:

1. **Format code**: `cargo fmt`
2. **Lint code**: `cargo clippy --all-targets -- -D warnings`
3. **Run tests**: `cargo test`
4. **Check REUSE compliance**: `reuse lint`
5. **Lint GitHub Actions** (after changes to `.github/workflows/*.yaml`): `actionlint`
6. **Check dependencies** (after changes to `Cargo.toml` or `Cargo.lock`): `cargo deny check`

All checks must pass before considering the work complete.

## Convenient Commands

You can run all checks at once using:

- `just all` - Run all checks individually (fmt, clippy, test, reuse, actionlint, deny)
- `just pre-commit` - Run pre-commit hooks on all files

To install pre-commit git hooks:

- `just pre-commit-install`

## Project-Specific Notes

- This project follows FSFE REUSE 3.3 specification
- All source files must have SPDX headers
- Files covered by `REUSE.toml` don't need individual headers
- License: Apache-2.0
- Copyright holder: Stackable GmbH
- All full sentences in comments (`//` and `///`) must end with a period

## Adding New Migration Rules

When adding new migration rules, follow these steps in order:

1. Create a new file in `src/migration/rules/` (e.g., `my_rule.rs`)
2. Implement the `MigrationRule` trait with SPDX headers
   - Rules can apply to processors, controller services, or both
   - The trait checks for `type` and `bundle` fields, not component type
3. Add the module to `src/migration/rules.rs` and export it
4. Register it in `Migrator::default()` in `src/migration.rs`
   - Add to the `rules` vec with appropriate comment (processor/controller service)
5. Add comprehensive unit tests in the rule file
   - Test both the rule in isolation and in the full migration flow
6. **Update README.md** in the "Supported Migrations" section:
   - Add a new subsection describing the migration
   - Include the old and new type/bundle values
   - Explain why the migration is needed (link to JIRA ticket if available)
7. Run all checks listed above (fmt, clippy, test, reuse lint)

All steps must be completed before considering the migration rule complete.

## Project Structure

- `src/main.rs` - CLI entry point, displays version detection and migration results
- `src/cli.rs` - Command-line argument parsing with clap
- `src/migration.rs` - Core migration engine with `Migrator` struct
- `src/migration/rules.rs` - Module declarations for all migration rules
- `src/migration/rules/*.rs` - Individual migration rule implementations
- There might be other files but these are the core ones
