use std::path::PathBuf;

use crate::{os::types::OSManager, parser::types::Project};

pub trait CompilerProperties {
    fn os_manager(&self) -> &Box<dyn OSManager>;
    fn input_file_path(&self) -> &PathBuf;
}

pub trait ProtobufCompiler: CompilerProperties {
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    fn ensure_dependencies(
        &self,
        deps: &Vec<&'static str>,
        version_flags: &Vec<&'static str>,
    ) -> Result<(), Vec<&'static str>> {
        let mut failed_commands: Vec<&'static str> = Vec::new();

        for i in 0..deps.len() {
            if let Err(_) = self
                .os_manager()
                .ensure_installation(&deps[i], &version_flags[i])
            {
                failed_commands.push(&deps[i]);
            }
        }

        if failed_commands.is_empty() {
            Ok(())
        } else {
            Err(failed_commands)
        }
    }

    fn assemble_compilation(
        &self,
        project: &Project,
        compiler_deps: Vec<&'static str>,
        compiler_version_flags: Vec<&'static str>,
        compiler_out_flag: &'static str,
        compiler_out_grpc_flag: &'static str,
        compiler_plugin: Option<&'static str>,
    ) -> Result<(), String> {
        if let Err(failed_dependencies) =
            self.ensure_dependencies(&compiler_deps, &compiler_version_flags)
        {
            let mut error_msg: String =
                String::from("Failed to check installation of the following dependencies: ");
            error_msg.push_str(failed_dependencies.join(",").as_str());

            return Err(error_msg);
        }

        let mut command_args: Vec<String> = Vec::new();

        for (key, value) in project.compile_options.iter() {
            if key == compiler_out_flag || key == compiler_out_grpc_flag {
                let err = format!(
                    "The argument {} do not must appear. Protoweld handle this using compiled_proto_folder option",
                    key
                );
                return Err(err);
            }
            let flag: String = format!("{}={}", key, value);
            command_args.push(flag);
        }

        command_args.push(format!(
            "{}={} {}={}",
            &compiler_out_flag,
            &project.compiled_proto_folder,
            compiler_out_grpc_flag,
            &project.compiled_proto_folder
        ));

        if let Some(plugin) = compiler_plugin {
            if let Some(plugin_path) = &project.plugin_path {
                let plugin_flag = format!("--plugin=protoc-gen-grpc={}", plugin_path);
                command_args.push(plugin_flag);
            } else {
                return Err(format!(
                    "The plugin {} must have a path in plugin_path option",
                    plugin
                ));
            }
        }

        for proto in project.associated_proto_files.iter() {
            command_args.push(String::from(proto));
        }

        if let Err(err) = self.os_manager().execute_command("protoc", &command_args) {
            return Err(err);
        }
        Ok(())
    }
}

pub struct GoCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
pub struct DotNetCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
pub struct RustCompiler {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
}
