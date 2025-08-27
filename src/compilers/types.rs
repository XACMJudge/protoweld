use crate::parser::types::Project;

pub trait ProtobufCompiler {
    fn compile_project(project: &Project) -> Result<(), String>;
}

pub struct GoCompiler;
pub struct DotNetCompiler;
pub struct RustCompiler;
