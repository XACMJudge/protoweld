//! Compiler parameter structures and type conversions.

use std::path::PathBuf;

use crate::{
    compilers::protobuf_compiler::ProtobufCompiler, os::types::OSManager, parser::types::Lang,
};

/// Parameters shared across all compiler implementations.
///
/// This structure contains the common data needed by all language-specific
/// compilers: the OS manager for system operations, the canonicalized input
/// file path for resolving relative paths, and the target language.
pub struct CompilerParams {
    /// OS manager for platform-specific operations
    pub os_manager: Box<dyn OSManager>,
    /// Canonicalized path to the configuration file
    pub input_file_path: PathBuf,
    /// Target programming language for compilation
    pub lang: Lang,
}

// NOTE: We need separate structs for each language because their compilation
// processes may vary significantly. For example, Rust requires post-processing
// to organize files into proper module structures.

/// Go language compiler implementation.
pub struct GoCompiler {
    /// Shared compiler parameters
    pub params: CompilerParams,
}

/// .NET (C#) language compiler implementation.
pub struct DotNetCompiler {
    /// Shared compiler parameters
    pub params: CompilerParams,
}

/// Rust language compiler implementation.
///
/// This compiler includes special post-processing to organize generated
/// files into proper Rust module structures.
pub struct RustCompiler {
    /// Shared compiler parameters
    pub params: CompilerParams,
}

/// Converts compiler parameters into the appropriate language-specific compiler.
///
/// This implementation uses the language field to determine which compiler
/// type to create. Each language has its own compiler struct to allow for
/// language-specific compilation logic.
impl From<CompilerParams> for Box<dyn ProtobufCompiler> {
    fn from(params: CompilerParams) -> Self {
        match params.lang {
            Lang::GoLang => Box::new(GoCompiler { params }),
            Lang::DotNet => Box::new(DotNetCompiler { params }),
            Lang::Rust => Box::new(RustCompiler { params }),
        }
    }
}
