//! ```text
//! cargo run -p colony-tokens -- generate   # rewrite generated/ from tokens/
//! cargo run -p colony-tokens -- check      # verify generated/ is up to date (CI)
//! ```

use std::process::ExitCode;

use anyhow::{bail, Result};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let root = colony_tokens::repo_root()?;
    let command = std::env::args().nth(1);

    match command.as_deref() {
        Some("generate") | None => {
            let report = colony_tokens::generate(&root)?;
            println!("{} families, {} variants", report.families, report.variants);
            for path in &report.written {
                println!("  wrote   {}", path.display());
            }
            for path in &report.removed {
                println!("  removed {}", path.display());
            }
            if report.written.is_empty() && report.removed.is_empty() {
                println!(
                    "  generated/ already up to date ({} files)",
                    report.unchanged
                );
            }
            Ok(ExitCode::SUCCESS)
        }
        Some("check") => {
            let problems = colony_tokens::check(&root)?;
            if problems.is_empty() {
                println!("generated/ is up to date with tokens/");
                return Ok(ExitCode::SUCCESS);
            }
            eprintln!("generated/ does not match tokens/:");
            for problem in &problems {
                eprintln!("  {problem}");
            }
            eprintln!("\nrun: cargo run -p colony-tokens -- generate");
            Ok(ExitCode::FAILURE)
        }
        Some(other) => bail!("unknown command {other:?} (expected `generate` or `check`)"),
    }
}
