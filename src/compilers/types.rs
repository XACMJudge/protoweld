use crate::{os::types::OSManager, parser::types::Project};

pub trait ProtobufCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    fn ensure_dependencies(&self) -> Result<(), Vec<String>>;
}

pub struct GoCompiler { pub os_manager: Box<dyn OSManager> }
pub struct DotNetCompiler { os_manager: Box<dyn OSManager> }
pub struct RustCompiler { os_manager: Box<dyn OSManager> }
