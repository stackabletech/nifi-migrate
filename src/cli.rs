// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Input flow.json file.
    #[arg(value_hint = ValueHint::FilePath)]
    pub input: PathBuf,

    /// Output flow.json file.
    #[arg(value_hint = ValueHint::FilePath)]
    pub output: PathBuf,

    /// Pretty-print the output JSON (default: compact).
    #[arg(short, long)]
    pub pretty: bool,

    /// Format-only mode: rewrite the file without applying migrations.
    /// This can help in diffing a file to see the changes as the formatting will have changed
    /// from the input. This way you'll have two consistently formatted files.
    #[arg(short, long)]
    pub format_only: bool,
}
