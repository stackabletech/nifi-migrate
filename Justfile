# SPDX-FileCopyrightText: 2025 Stackable GmbH
# SPDX-License-Identifier: Apache-2.0

# Run all checks (same as pre-commit)
all: fmt clippy test reuse actionlint deny

# Format code
fmt:
    cargo fmt

# Lint code
clippy:
    cargo clippy --all-targets -- -D warnings

# Run tests
test:
    cargo test

# Check REUSE compliance
reuse:
    reuse lint

# Lint GitHub Actions workflows
actionlint:
    actionlint

# Check dependencies
deny:
    cargo deny check

# Run pre-commit hooks on all files
pre-commit:
    pre-commit run \
      --all-files \
      --verbose \
      --show-diff-on-failure \
      --color always

# Install pre-commit hooks
pre-commit-install:
    pre-commit install

# Build release binary
build:
    cargo build --release

# Run the tool
run input output="flow-migrated.json" *args="":
    cargo run -- --input "{{input}}" --output "{{output}}" {{args}}
