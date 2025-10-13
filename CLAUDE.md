<!--
SPDX-FileCopyrightText: 2025 Stackable GmbH
SPDX-License-Identifier: Apache-2.0
-->

# Claude Code Instructions

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
- **All full sentences in comments (`//` and `///`) must end with a period**

## Adding New Migration Rules

When adding new migration rules, follow these steps in order:

1. Create a new file in `src/rules/` (e.g., `my_rule.rs`)
2. Implement the `MigrationRule` trait with SPDX headers
3. Add the module to `src/rules/mod.rs` and export it
4. Register it in `Migrator::new()` in `src/lib.rs`
5. Add comprehensive tests in the rule file
6. **Update README.md** in the "Supported Migrations" section:
   - Add a new subsection describing the migration
   - Include the old and new type/bundle values
   - Explain why the migration is needed
7. Run all checks listed above (fmt, clippy, test, reuse lint)

All steps must be completed before considering the migration rule complete.
