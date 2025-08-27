use clap::Parser;
use log::info;
use protoweld::{parser::types::{IProtoweldParser, ProtoweldParser}, types::cli::Cli};

fn main() {
    env_logger::init();

    let args = Cli::parse();

    let result = ProtoweldParser::parse(&args.filename);

    if let Err(error) = result {
        panic!("[PROTOWELD] Parser throw a error: {}", error)
    }

    let parser = result.unwrap();

    println!("{:?}", &parser);

    info!("Esta es una información de log")
}
