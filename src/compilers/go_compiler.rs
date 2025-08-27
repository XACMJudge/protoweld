use crate::{
    compilers::types::{GoCompiler, ProtobufCompiler},
    parser::types::Project,
};

impl ProtobufCompiler for GoCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), String> {
        Ok(())
    }

    fn ensure_dependencies(&self) -> Result<(), Vec<String>> {
        Ok(())
    }
}
