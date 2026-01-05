//! Main executor for generating proto files across all configured projects.

use log::info;

use crate::{
    compilers::{protobuf_compiler::ProtobufCompiler, shared::get_compiler},
    parser::types::ProtoweldParser,
};

/// Generates proto files for all projects defined in the configuration.
///
/// This function iterates through all active projects in the configuration,
/// selects the appropriate compiler based on the target language, and
/// compiles all associated proto files for each project.
///
/// # Arguments
///
/// * `parser` - The parsed Protoweld configuration containing all project definitions
/// * `base_path` - The base path of the configuration file (used for resolving relative paths)
///
/// # Returns
///
/// * `Ok(())` - All projects compiled successfully
/// * `Err(String)` - Error message if compilation fails for any project
///
/// # Process
///
/// 1. Iterates through each project in `active_projects`
/// 2. Gets the appropriate compiler for the project's language
/// 3. Compiles all proto files for the project
/// 4. Logs success for each compiled project
/// 5. Returns early on first error
///
/// # Errors
///
/// This function will return an error if:
/// - The language is not supported
/// - Required dependencies are missing
/// - Proto file compilation fails
/// - File system operations fail
pub fn generate_protos(parser: &ProtoweldParser, base_path: &String) -> Result<(), String> {
    for project in parser.active_projects.iter() {
        let compiler_result = get_compiler(&project.lang, base_path);
        if let Err(_) = compiler_result {
            return Err(String::from("[PROTOWELD] Lang parsed do not supported."));
        }

        let compiler: Box<dyn ProtobufCompiler> = compiler_result.unwrap();
        let compilation_result = compiler.compile_project(&project);

        if let Err(error) = compilation_result {
            return Err(error);
        }

        info!("Compiled project {}", project.path);
    }
    Ok(())
}
