use std::path::PathBuf;

use crate::compilers::{
    langs_compilers::compiler_types::DotNetCompiler,
    protobuf_compiler::{CompilerProperties, ProtobufCompiler},
};

impl CompilerProperties for DotNetCompiler {
    fn os_manager(&self) -> &Box<dyn crate::os::types::OSManager> {
        &self.params.os_manager
    }

    fn input_file_path(&self) -> &PathBuf {
        &self.params.input_file_path
    }
}

static DOTNET_DEPS: [&'static str; 2] = ["protoc", "dotnet"];
static DOTNET_VERSION_FLAGS: [&'static str; 2] = ["--version", "--version"];

static DOTNET_MESSAGES_OUT_ARGUMENT: &'static str = "--csharp_out";
static DOTNET_GRPC_OUT_ARGUMENT: &'static str = "--grpc_out";
static DOTNET_PLUGIN_NAME: &'static str = "grpc_csharp_plugin";

impl ProtobufCompiler for DotNetCompiler {
    fn compile_project(&self, project: &crate::parser::types::Project) -> Result<(), String> {
        self.assemble_compilation(
            project,
            DOTNET_DEPS.to_vec(),
            DOTNET_VERSION_FLAGS.to_vec(),
            DOTNET_MESSAGES_OUT_ARGUMENT,
            DOTNET_GRPC_OUT_ARGUMENT,
            Option::Some(DOTNET_PLUGIN_NAME),
        )
    }
}
