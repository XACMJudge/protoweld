//! Type definitions for Protoweld configuration structures.

use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

/// Trait for parsing Protoweld configuration files.
///
/// Implementations of this trait are responsible for reading and deserializing
/// YAML configuration files into `ProtoweldParser` structures.
pub trait IProtoweldParser {
    /// Parses a YAML configuration file and returns a `ProtoweldParser` instance.
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to the YAML configuration file
    ///
    /// # Returns
    ///
    /// * `Ok(ProtoweldParser)` - Successfully parsed configuration
    /// * `Err(String)` - Error message if parsing fails
    fn parse(filename: &str) -> Result<ProtoweldParser, String>;
}

/// Supported programming languages for proto file compilation.
///
/// Each variant corresponds to a different language-specific compiler
/// that generates code from `.proto` files.
#[derive(Debug, PartialEq, Deserialize, Copy, Clone)]
pub enum Lang {
    /// Go programming language
    GoLang,
    /// .NET (C#) programming language
    DotNet,
    /// Rust programming language
    Rust,
}

impl FromStr for Lang {
    type Err = &'static str;

    /// Converts a string to a `Lang` enum variant.
    ///
    /// # Arguments
    ///
    /// * `s` - String representation of the language (e.g., "GoLang", "DotNet", "Rust")
    ///
    /// # Returns
    ///
    /// * `Ok(Lang)` - Successfully parsed language
    /// * `Err(&'static str)` - Error if the language is not supported
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GoLang" => Ok(Lang::GoLang),
            "DotNet" => Ok(Lang::DotNet),
            "Rust" => Ok(Lang::Rust),
            _ => Err("Unsupported lang"),
        }
    }
}

/// Configuration for a single project that needs proto file compilation.
///
/// This structure defines all the settings needed to compile `.proto` files
/// for a specific project, including the target language, proto file paths,
/// output directory, and compilation options.
#[derive(Debug, Deserialize)]
pub struct Project {
    /// Unique identifier for the project (used for logging and identification)
    pub path: String,
    /// Output directory where compiled proto files will be placed
    pub compiled_proto_folder: String,
    /// List of paths to `.proto` files that should be compiled for this project
    pub associated_proto_files: Vec<String>,
    /// Optional path to a custom gRPC plugin (required for .NET projects)
    pub plugin_path: Option<String>,
    /// Target programming language for code generation
    pub lang: Lang,
    /// Additional compilation options passed to `protoc`
    ///
    /// These are key-value pairs where keys are protoc flags (e.g., "-I", "--include_imports")
    /// and values are the flag values. Empty strings indicate flags without values.
    #[serde(default)]
    pub compile_options: HashMap<String, String>
}

/// Root structure representing the entire Protoweld configuration.
///
/// This structure is deserialized from the YAML configuration file and contains
/// all projects that need proto file compilation.
#[derive(Debug, Deserialize)]
pub struct ProtoweldParser {
    /// List of all projects configured for proto file compilation
    pub active_projects: Vec<Project>,
}
