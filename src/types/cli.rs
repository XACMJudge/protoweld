//! Command-line interface argument definitions for Protoweld.

use clap::Parser;

/// Command-line arguments structure for Protoweld.
///
/// This struct defines the CLI interface using `clap` for argument parsing.
/// The version and about information are automatically read from `Cargo.toml`.
#[derive(Parser, Debug)]
#[command(version, about)] // Read from Cargo.toml
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to the YAML configuration file containing project definitions.
    ///
    /// This file should contain the `active_projects` array with all projects
    /// that need to have their `.proto` files compiled.
    #[arg(short, long)]
    pub filename: String,
}
