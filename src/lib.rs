//! Protoweld - A tool for automating Protocol Buffer compilation across multiple projects.
//!
//! Protoweld simplifies the process of compiling `.proto` files into language-specific code
//! for multiple projects and programming languages. It supports Go, .NET (C#), and Rust,
//! and can handle multiple projects with different configurations in a single run.
//!
//! # Example
//!
//! ```no_run
//! use protoweld::parser::types::{IProtoweldParser, ProtoweldParser};
//!
//! let parser = ProtoweldParser::parse("config.yaml")?;
//! // Use parser to generate proto files...
//! ```

pub mod types;
pub mod parser;
pub mod executor;
pub mod compilers;
pub mod os;
