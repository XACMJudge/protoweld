use log::info;

use crate::{
    compilers::{shared::get_compiler, types::ProtobufCompiler},
    parser::types::ProtoweldParser,
};

pub fn generate_protos(parser: &ProtoweldParser) -> Result<(), &'static str> {
    for project in parser.active_projects.iter() {
        let compiler_result = get_compiler(&project.lang);
        if let Err(_) = compiler_result {
            panic!("[PROTOWELD] Lang parsed do not supported.")
        }

        let compiler: Box<dyn ProtobufCompiler> = compiler_result.unwrap();
        let compilation_result = compiler.compile_project(&project);

        if let Err(error) = compilation_result {
            return Err(error);
        }

        info!(
            "[PROTOWELD - COMPILATION] Compiled project {}",
            project.path
        );
    }
    Ok(())
}
