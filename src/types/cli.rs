use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)] // Read from Cargo.toml
#[command(propagate_version = true)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init
}
