use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about)] // Read from Cargo.toml
#[command(propagate_version = true)]
pub struct Cli {

    #[arg(short, long)]
    pub filename: String

}
