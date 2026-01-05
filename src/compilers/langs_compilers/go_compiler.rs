//! Go language compiler implementation.

use std::path::PathBuf;

use crate::{
    compilers::{
        langs_compilers::compiler_types::GoCompiler,
        protobuf_compiler::{CompilerProperties, ProtobufCompiler},
    },
    parser::types::Project,
};

/// Required dependencies for Go proto compilation.
///
/// These tools must be installed and available in PATH:
/// - `protoc`: Protocol Buffers compiler
/// - `go`: Go toolchain
/// - `protoc-gen-go`: Go protobuf code generator
/// - `protoc-gen-go-grpc`: Go gRPC code generator
static GO_DEPS: [&'static str; 4] = ["protoc", "go", "protoc-gen-go", "protoc-gen-go-grpc"];

/// Version flags for checking each dependency.
///
/// Each flag corresponds to the dependency at the same index in `GO_DEPS`.
static GO_VERSION_FLAGS: [&'static str; 4] = ["--version", "version", "--version", "--version"];

/// Protoc flag for generating Go message types.
static GO_MESSAGES_OUT_ARGUMENT: &'static str = "--go_out";

/// Protoc flag for generating Go gRPC service code.
static GO_GRPC_OUT_ARGUMENT: &'static str = "--go-grpc_out";

impl CompilerProperties for GoCompiler {
    fn os_manager(&self) -> &Box<dyn crate::os::types::OSManager> {
        &self.params.os_manager
    }

    fn input_file_path(&self) -> &PathBuf {
        &self.params.input_file_path
    }
}

impl ProtobufCompiler for GoCompiler {
    /// Compiles proto files to Go code.
    ///
    /// This implementation uses `protoc-gen-go` for message types and
    /// `protoc-gen-go-grpc` for gRPC services. No custom plugin is required.
    ///
    /// # Arguments
    ///
    /// * `project` - Project configuration with Go-specific settings
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Go code generated successfully
    /// * `Err(String)` - Error message if compilation fails
    ///
    /// # Generated Output
    ///
    /// Generates `.pb.go` files for message types and `_grpc.pb.go` files
    /// for gRPC services in the specified `compiled_proto_folder`.
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
