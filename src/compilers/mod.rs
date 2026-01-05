//! Compiler module for language-specific Protocol Buffer code generation.
//!
//! This module provides the infrastructure for compiling `.proto` files into
//! language-specific code. It includes a base compiler trait, language-specific
//! implementations, and a factory for creating appropriate compilers.

pub mod protobuf_compiler;
pub mod shared;
pub mod langs_compilers;
