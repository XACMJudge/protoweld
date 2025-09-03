use std::{env, error::Error, fs, path::PathBuf};

use log::info;

use crate::{
    compilers::types::{CompilerProperties, GoCompiler, ProtobufCompiler},
    parser::types::Project,
};

static GO_DEPS: [&'static str; 4] = ["protoc", "go", "protoc-gen-go", "protoc-gen-go-grpc"];
static GO_VERSION_FLAGS: [&'static str; 4] = ["--version", "version", "--version", "--version"];

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
        if let Err(failed_dependencies) =
            self.ensure_dependencies(&GO_DEPS.to_vec(), &GO_VERSION_FLAGS.to_vec())
        {
            let mut error_msg: String =
                String::from("Failed to check installation of the following dependencies: ");
            error_msg.push_str(failed_dependencies.join(",").as_str());

            return Err(error_msg);
        }

        info!("YAML input path {}", self.input_file_path.display());
        let mut command_args: Vec<String> = Vec::new();

        for (key, value) in project.compile_options.iter() {
            let flag : String = format!("{}={}", key, value);
            command_args.push(flag);
        }

        for proto in project.asociated_proto_files.iter() {
            let dest_proto: PathBuf = self.input_file_path.join(PathBuf::from(proto));
            let canonical_result = fs::canonicalize(&dest_proto);

            if let Err(err) = canonical_result {
                return Err(format!(
                    "Failed to get canonical path {} with error {}",
                    proto, err
                ));
            }

            command_args.push(canonical_result.unwrap().to_str().unwrap().into());
            info!("Last path pushed: {}", command_args.last().unwrap());
        }

        Ok(())
    }
}
