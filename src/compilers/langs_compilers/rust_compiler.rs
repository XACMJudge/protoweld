use std::path::PathBuf;

use crate::compilers::{
    langs_compilers::compiler_types::RustCompiler,
    protobuf_compiler::{CompilerProperties, ProtobufCompiler},
};

impl CompilerProperties for RustCompiler {
    fn os_manager(&self) -> &Box<dyn crate::os::types::OSManager> {
        &self.params.os_manager
    }

    fn input_file_path(&self) -> &PathBuf {
        &self.params.input_file_path
    }
}

static RUST_DEPS: [&'static str; 3] = ["protoc", "protoc-gen-tonic", "protoc-gen-prost"];
static RUST_VERSION_FLAGS: [&'static str; 3] = ["--version", "", ""];

static RUST_MESSAGES_OUT_ARGUMENT: &'static str = "--prost_out";
static RUST_GRPC_OUT_ARGUMENT: &'static str = "--tonic_out";

impl ProtobufCompiler for RustCompiler {
    fn compile_project(&self, project: &crate::parser::types::Project) -> Result<(), String> {
        let result = self.assemble_compilation(
            project,
            RUST_DEPS.to_vec(),
            RUST_VERSION_FLAGS.to_vec(),
            RUST_MESSAGES_OUT_ARGUMENT,
            RUST_GRPC_OUT_ARGUMENT,
            Option::None,
        );

        if let Err(e) = result {
            return Err(e);
        }

        return Ok(());
    }
}
