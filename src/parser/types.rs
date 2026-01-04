use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

pub trait IProtoweldParser {
    fn parse(filename: &str) -> Result<ProtoweldParser, String>;
}

#[derive(Debug, PartialEq, Deserialize, Copy, Clone)]
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
    pub associated_proto_files: Vec<String>,
    pub plugin_path: Option<String>,
    pub lang: Lang,
    #[serde(default)]
    pub compile_options: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
pub struct ProtoweldParser {
    pub active_projects: Vec<Project>,
}
