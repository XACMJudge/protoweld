use clap::Parser;
use log::{error, info};
use protoweld::types::{
    cli::{Cli, Commands},
    init_command::{InitActions, InitCommand},
};

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let res = InitCommand::init();
            match res {
                Ok(_) => println!(),
                Err(e) => error!("{}", e),
            }
        }
    };

    info!("Esta es una información de log")
}
