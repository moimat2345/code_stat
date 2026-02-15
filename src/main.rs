use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;

use code_stats::analyzer;
use code_stats::report;
use code_stats::stats;

/// Analyze source code repositories and display language statistics.
///
/// Recursively scans a directory, detects programming languages,
/// and reports file counts, line counts, and size breakdowns.
#[derive(Parser)]
#[command(name = "code-stats", version, about)]
struct Cli {
    /// Path to the directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = cli
        .path
        .canonicalize()
        .with_context(|| format!("cannot access '{}'", cli.path.display()))?;

    if !path.is_dir() {
        bail!("'{}' is not a directory", path.display());
    }

    let scan_result = analyzer::analyze(&path);
    let report_data = stats::aggregate(&scan_result);
    report::render(&report_data);

    Ok(())
}
