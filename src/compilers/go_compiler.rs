use std::{env, fs, path::PathBuf};

use log::info;

use crate::{
    compilers::types::{GoCompiler, ProtobufCompiler},
    parser::types::Project,
};

static GO_DEPS: [&'static str; 4] = ["protoc", "go", "protoc-gen-go", "protoc-gen-go-grpc"];
static GO_VERSION_FLAGS: [&'static str; 4] = ["--version", "version", "--version", "--version"];

impl ProtobufCompiler for GoCompiler {
    fn compile_project(&self, project: &Project) -> Result<(), String> {
        if let Err(failed_dependencies) = self.ensure_dependencies() {
            let mut error_msg: String =
                String::from("Failed to check installation of the following dependencies: ");
            error_msg.push_str(failed_dependencies.join(",").as_str());

            return Err(error_msg);
        }

        info!("YAML input path {}", self.input_file_path.display());

        for proto in project.asociated_proto_files.iter() {
            
            // "protoc --go_out=./database-server/server --go-grpc_out=./database-server/server entities/protos/database-server/operations.proto"
            
            let mut command_args : Vec<String> = Vec::new();

            let dest_directory : PathBuf = self.input_file_path.join(PathBuf::from(proto));
            let canonical_result = fs::canonicalize(&dest_directory);

            // TODO: Canonical path to directory and files


            command_args.push(
                format!("--go_out={}", &self.input_file_path.to_str().unwrap())
                );
        }

        Ok(())
    }

    fn ensure_dependencies(&self) -> Result<(), Vec<&'static str>> {
        let mut failed_commands: Vec<&'static str> = Vec::new();

        for i in 0..GO_DEPS.len() {
            if let Err(_) = self
                .os_manager
                .ensure_installation(&GO_DEPS[i], &GO_VERSION_FLAGS[i])
            {
                failed_commands.push(&GO_DEPS[i]);
            }
        }

        if failed_commands.is_empty() {
            Ok(())
        } else {
            Err(failed_commands)
        }
    }
}
