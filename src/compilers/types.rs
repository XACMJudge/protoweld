use std::path::PathBuf;

use crate::{os::types::OSManager, parser::types::Project};

pub trait CompilerProperties {
    fn os_manager(&self) -> &Box<dyn OSManager>;
    fn input_file_path(&self) -> &PathBuf;
}

pub trait ProtobufCompiler: CompilerProperties {
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    fn ensure_dependencies(
        &self,
        deps: &Vec<&'static str>,
        version_flags: &Vec<&'static str>,
    ) -> Result<(), Vec<&'static str>> {
        let mut failed_commands: Vec<&'static str> = Vec::new();

        for i in 0..deps.len() {
            if let Err(_) = self
                .os_manager()
                .ensure_installation(&deps[i], &version_flags[i])
            {
                failed_commands.push(&deps[i]);
            }
        }

        if failed_commands.is_empty() {
            Ok(())
        } else {
            Err(failed_commands)
        }
    }
}

pub struct GoCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
pub struct DotNetCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
pub struct RustCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
