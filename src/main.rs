use clap::Parser;
use protoweld::{
    executor::protoweld_executor::generate_protos,
    parser::types::{IProtoweldParser, ProtoweldParser},
    types::cli::Cli,
};

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
