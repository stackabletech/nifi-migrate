// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nifi-migrate")]
#[command(version)]
#[command(about = "Migrate NiFi 1.x flow.json files to NiFi 2.x format", long_about = None)]
pub struct Args {
    /// Input flow.json file
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output flow.json file
    #[arg(short, long)]
    pub output: PathBuf,

    /// Pretty-print the output JSON (default: compact)
    #[arg(short, long)]
    pub pretty: bool,
}
