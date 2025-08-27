use serde::Deserialize;
use std::str::FromStr;

pub trait IProtoweldParser {
    fn parse(filename: &str) -> Result<ProtoweldParser, String>;
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Lang {
    GoLang,
    DotNet,
    Rust,
}

impl FromStr for Lang {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GoLang" => Ok(Lang::GoLang),
            "DotNet" => Ok(Lang::DotNet),
            "Rust" => Ok(Lang::Rust),
            _ => Err("Unsupported lang"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub path: String,
    pub compiled_proto_folder: String,
    pub asociated_proto_files: Vec<String>,
    pub lang: Lang,
}

#[derive(Debug, Deserialize)]
pub struct ProtoweldParser {
    pub active_projects: Vec<Project>,
}
