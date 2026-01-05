//! Main entry point for the Protoweld command-line tool.
//!
//! This module handles CLI argument parsing, configuration file loading,
//! and orchestrates the proto file compilation process.

use clap::Parser;
use protoweld::{
    executor::protoweld_executor::generate_protos,
    parser::types::{IProtoweldParser, ProtoweldParser},
    types::cli::Cli,
};

/// Main entry point for Protoweld.
///
/// This function:
/// 1. Initializes the logger
/// 2. Parses command-line arguments
/// 3. Loads and parses the YAML configuration file
/// 4. Generates proto files for all configured projects
/// 5. Reports success or failure
///
/// # Panics
///
/// This function will panic if:
/// - The configuration file cannot be read or parsed
/// - Proto file generation fails for any project
fn main() {
    env_logger::init();

    let args = Cli::parse();

    let result = ProtoweldParser::parse(&args.filename);

    if let Err(error) = result {
        panic!("[PROTOWELD] Parser throw a error: {}", error)
    }

    let parser = result.unwrap();
    let generation_result = generate_protos(&parser, &args.filename);

    if let Err(error) = generation_result {
        panic!("[PROTOWELD] Generation failed. {}", error)
    }

    println!("[PROTOWELD] Generation completed.")
}
