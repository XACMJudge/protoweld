use std::path::PathBuf;

/*use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    name: Option<String>
}*/
use crate::types::custom_error::Result;

pub trait InitActions {
    fn init() -> Result<()>;
    fn build_hash(file_path: &Vec<PathBuf>) -> Result<()>;
}

pub struct InitCommand;
