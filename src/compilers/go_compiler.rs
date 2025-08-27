use crate::{
    compilers::types::{GoCompiler, ProtobufCompiler},
    parser::types::Project,
};

static GO_DEPS: [&'static str; 1] = ["protoc"];

impl ProtobufCompiler for GoCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), &'static str> {
        Ok(())
    }

    fn ensure_dependencies(&self) -> Result<(), Vec<&'static str>> {
        let mut failed_commands: Vec<&'static str> = Vec::new();

        for dependency in &GO_DEPS {
            if let Err(_) = self.os_manager.ensure_installation(&dependency) {
                failed_commands.push(dependency);
            }
        }

        if failed_commands.is_empty() {
            Ok(())
        } else {
            Err(failed_commands)
        }
    }
}
