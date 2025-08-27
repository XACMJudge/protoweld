use crate::{os::types::OSManager, parser::types::Project};

pub trait ProtobufCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), &'static str>;
    fn ensure_dependencies(&self) -> Result<(), Vec<&'static str>>;
}

pub struct GoCompiler {
    pub os_manager: Box<dyn OSManager>,
}
pub struct DotNetCompiler {
    os_manager: Box<dyn OSManager>,
}
pub struct RustCompiler {
    os_manager: Box<dyn OSManager>,
}
