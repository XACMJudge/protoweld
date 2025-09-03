use std::path::PathBuf;

use crate::compilers::types::{CompilerProperties, ProtobufCompiler, RustCompiler};

impl CompilerProperties for RustCompiler {
    fn os_manager(&self) -> &Box<dyn crate::os::types::OSManager> {
        &self.os_manager
    }

    fn input_file_path(&self) -> &PathBuf {
        &self.input_file_path
    }
}

static RUST_DEPS: [&'static str; 4] = [
    "protoc",
    "protoc-gen-rs",
    "protoc-gen-tonic",
    "protoc-gen-prost",
];
static RUST_VERSION_FLAGS: [&'static str; 4] = ["--version", "", "", ""];

static RUST_MESSAGES_OUT_ARGUMENT: &'static str = "--rs_out";
static RUST_GRPC_OUT_ARGUMENT: &'static str = "--tonic_out";

impl ProtobufCompiler for RustCompiler {
    fn compile_project(&self, project: &crate::parser::types::Project) -> Result<(), String> {
        self.assemble_compilation(
            project,
            RUST_DEPS.to_vec(),
            RUST_VERSION_FLAGS.to_vec(),
            RUST_MESSAGES_OUT_ARGUMENT,
            RUST_GRPC_OUT_ARGUMENT,
            Option::None,
        )
    }
}
