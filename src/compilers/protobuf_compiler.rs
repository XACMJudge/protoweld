use std::{collections::HashSet, path::PathBuf};

use crate::{os::types::OSManager, parser::types::Project};

static PACKAGE_KEYWORD: &'static str = "package";
static SEMICOLON: &'static str = ";";

pub trait CompilerProperties {
    fn os_manager(&self) -> &Box<dyn OSManager>;
    fn input_file_path(&self) -> &PathBuf;
}

pub trait ProtobufCompiler: CompilerProperties {
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    fn get_packages_set(&self, protos: &Vec<String>) -> Result<HashSet<String>, String> {
        let mut result: HashSet<String> = HashSet::new();
        for proto in protos.iter() {
            let find_package = self
                .os_manager()
                .grep(PathBuf::from(&proto), PACKAGE_KEYWORD);
            if let Err(e) = find_package {
                return Err(e);
            }

            let grep_result: (String, usize) = find_package.unwrap();

            if grep_result.1 == usize::MAX {
                return Err(String::from(format!(
                    "Package keyword missing in {}",
                    &proto
                )));
            }

            let cut_string = grep_result
                .0
                .clone()
                .split_off(grep_result.1)
                .split_off(PACKAGE_KEYWORD.len() + 1);

            let mut semicolon_split = cut_string.split(SEMICOLON);

            match semicolon_split.next() {
                Some(name) => result.insert(name.into()),
                None => return Err(String::from("Bad .proto structure - Keyword package")),
            };
        }

        return Ok(result);
    }
    fn ensure_dependencies(
        &self,
        deps: &Vec<&'static str>,
        version_flags: &Vec<&'static str>,
    ) -> Result<(), Vec<&'static str>> {
        let mut failed_commands: Vec<&'static str> = Vec::new();

        for i in 0..deps.len() {
            if let Err(_) = self.os_manager().execute_command(
                &deps[i],
                &vec![String::from(version_flags[i])],
                true,
            ) {
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

            let flag = match value == "" {
                true => format!("{}", key),
                false => format!("{}={}", key, value),
            };
            command_args.push(flag);
        }

        command_args.push(format!(
            "{}={}",
            &compiler_out_flag, &project.compiled_proto_folder,
        ));

        command_args.push(format!(
            "{}={}",
            &compiler_out_grpc_flag, &project.compiled_proto_folder,
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

        if let Err(err) = self
            .os_manager()
            .execute_command("protoc", &command_args, false)
        {
            return Err(err);
        }
        Ok(())
    }
}
