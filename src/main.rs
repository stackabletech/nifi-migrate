// SPDX-FileCopyrightText: 2025 Stackable GmbH
// SPDX-License-Identifier: Apache-2.0

mod cli;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use nifi_migrate::Migrator;

fn main() -> Result<()> {
    let args = Args::parse();

    let migrator = Migrator::new();
    // Should we get more arguments we can pass along a struct...this kinda grew organically :)
    let changes =
        migrator.migrate_file(&args.input, &args.output, args.pretty, args.format_only)?;

    if args.format_only {
        println!("Format-only mode: File reformatted without migrations.");
        println!("Output written to: {}", args.output.display());
    } else if changes.is_empty() {
        println!("No migrations needed.");
    } else {
        println!("Migration complete. Changes made:");

        for change in &changes {
            println!(
                "  - {} ({}): {}",
                change.processor_name, change.processor_id, change.rule_description
            );
        }

        println!("\nOutput written to: {}", args.output.display());
    }

    Ok(())
}
