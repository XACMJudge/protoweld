//! .NET (C#) language compiler implementation.

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

/// Required dependencies for .NET proto compilation.
///
/// These tools must be installed and available:
/// - `protoc`: Protocol Buffers compiler
/// - `dotnet`: .NET SDK
static DOTNET_DEPS: [&'static str; 2] = ["protoc", "dotnet"];

/// Version flags for checking each dependency.
static DOTNET_VERSION_FLAGS: [&'static str; 2] = ["--version", "--version"];

/// Protoc flag for generating C# message types.
static DOTNET_MESSAGES_OUT_ARGUMENT: &'static str = "--csharp_out";

/// Protoc flag for generating C# gRPC service code.
static DOTNET_GRPC_OUT_ARGUMENT: &'static str = "--grpc_out";

/// Name of the gRPC C# plugin (used for error messages).
static DOTNET_PLUGIN_NAME: &'static str = "grpc_csharp_plugin";

impl ProtobufCompiler for DotNetCompiler {
    /// Compiles proto files to C# code.
    ///
    /// This implementation uses `protoc` with `--csharp_out` for message types
    /// and requires the `grpc_csharp_plugin` for gRPC services. The plugin path
    /// must be specified in the project's `plugin_path` field.
    ///
    /// # Arguments
    ///
    /// * `project` - Project configuration with .NET-specific settings
    ///
    /// # Returns
    ///
    /// * `Ok(())` - C# code generated successfully
    /// * `Err(String)` - Error message if compilation fails
    ///
    /// # Requirements
    ///
    /// The project configuration must include `plugin_path` pointing to the
    /// `grpc_csharp_plugin` executable, typically found in:
    /// `~/.nuget/packages/grpc.tools/<version>/tools/<platform>/grpc_csharp_plugin`
    ///
    /// # Generated Output
    ///
    /// Generates `.cs` files for message types and gRPC services in the
    /// specified `compiled_proto_folder`.
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
