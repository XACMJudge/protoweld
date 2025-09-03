use std::path::PathBuf;

use crate::{
    compilers::types::{CompilerProperties, GoCompiler, ProtobufCompiler},
    parser::types::Project,
};

static GO_DEPS: [&'static str; 4] = ["protoc", "go", "protoc-gen-go", "protoc-gen-go-grpc"];
static GO_VERSION_FLAGS: [&'static str; 4] = ["--version", "version", "--version", "--version"];

static GO_MESSAGES_OUT_ARGUMENT: &'static str = "--go_out";
static GO_GRPC_OUT_ARGUMENT: &'static str = "--go-grpc_out";

impl CompilerProperties for GoCompiler {
    fn os_manager(&self) -> &Box<dyn crate::os::types::OSManager> {
        &self.os_manager
    }

    fn input_file_path(&self) -> &PathBuf {
        &self.input_file_path
    }
}

impl ProtobufCompiler for GoCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), String> {
        self.assemble_compilation(
            project,
            GO_DEPS.to_vec(),
            GO_VERSION_FLAGS.to_vec(),
            GO_MESSAGES_OUT_ARGUMENT,
            GO_GRPC_OUT_ARGUMENT,
            Option::None,
        )
    }
}
