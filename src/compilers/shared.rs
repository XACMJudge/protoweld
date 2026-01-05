//! Factory function for creating language-specific compilers.

use std::{fs, path::PathBuf};

use log::error;

use crate::{
    compilers::{langs_compilers::compiler_types::CompilerParams, protobuf_compiler::ProtobufCompiler},
    os::shared::get_os_manager,
    parser::types::Lang,
};

/// Creates an appropriate compiler for the specified language.
///
/// This function sets up the compiler with the necessary OS manager and
/// canonicalizes the base file path to ensure consistent path resolution
/// for relative proto file paths.
///
/// # Arguments
///
/// * `lang` - The target programming language
/// * `base_file_path` - Path to the configuration file (used for resolving relative paths)
///
/// # Returns
///
/// * `Ok(Box<dyn ProtobufCompiler>)` - Language-specific compiler instance
/// * `Err(&'static str)` - Error if OS manager cannot be created or path canonicalization fails
///
/// # Process
///
/// 1. Gets the OS manager for the current platform
/// 2. Canonicalizes the base file path to an absolute path
/// 3. Creates compiler parameters with the language, path, and OS manager
/// 4. Converts parameters into the appropriate compiler type
pub fn get_compiler(
    lang: &Lang,
    base_file_path: &String,
) -> Result<Box<dyn ProtobufCompiler>, &'static str> {
    let platform_result = get_os_manager();

    if let Err(error) = platform_result {
        return Err(error);
    }

    let platform_manager = platform_result.unwrap();
    let base_path_buf = PathBuf::from(base_file_path);
    let canonical_path_result = fs::canonicalize(&base_path_buf);

    if let Err(error) = &canonical_path_result {
        error!("Canonical path failed {}", error);
        return Err("Canonicalize base path failed");
    }

    let params = CompilerParams {
        lang: *lang,
        input_file_path: canonical_path_result.unwrap(),
        os_manager: platform_manager,
    };

    return Ok(params.into());
}
