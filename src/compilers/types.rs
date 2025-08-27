use std::path::PathBuf;

use crate::{os::types::OSManager, parser::types::Project};

pub trait ProtobufCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    fn ensure_dependencies(&self) -> Result<(), Vec<&'static str>>;
}

pub struct GoCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf
}
pub struct DotNetCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf
}
pub struct RustCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf
}
