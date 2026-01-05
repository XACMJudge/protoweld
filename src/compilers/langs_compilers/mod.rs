//! Language-specific compiler implementations.
//!
//! This module contains implementations of `ProtobufCompiler` for each
//! supported programming language. Each compiler handles language-specific
//! compilation requirements and post-processing.

pub mod go_compiler;
pub mod dotnet_compiler;
pub mod rust_compiler;
pub mod compiler_types;
