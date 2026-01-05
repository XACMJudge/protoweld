use std::{collections::HashSet, path::PathBuf};

use log::debug;

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

static INCLUDE_MACRO_PLACEHOLDER: &'static str = "package_tonic";
static RUST_INCLUDE_MACRO: &'static str = "include!(\"package_tonic\");";
static TONIC_USE_SUPER_DIRECTIVE: &'static str = "use super::package_tonic::*;";
static RUST_STANDARD_MODULE_FILENAME: &'static str = "mod.rs";

impl ProtobufCompiler for RustCompiler {
    fn compile_project(&self, project: &crate::parser::types::Project) -> Result<(), String> {
        let packages: HashSet<String> = self.get_packages_set(&project.associated_proto_files)?;

        self.assemble_compilation(
            project,
            RUST_DEPS.to_vec(),
            RUST_VERSION_FLAGS.to_vec(),
            RUST_MESSAGES_OUT_ARGUMENT,
            RUST_GRPC_OUT_ARGUMENT,
            Option::None,
        )?;

        let base_path: PathBuf = (&project.compiled_proto_folder).into();

        // See https://WikiExplainingThis.com
        for pkg in packages.iter() {
            let mut prost_file: PathBuf = base_path.clone();
            prost_file.push(pkg);

            let mut module_path: PathBuf = prost_file.clone();

            prost_file.push(format!("{}.rs", &pkg));

            let mut bad_tonic_file: PathBuf = base_path.clone();
            bad_tonic_file.push(pkg);

            let mut good_tonic_file: PathBuf = bad_tonic_file.clone();
            let bad_tonic_filename: String = format!("{}.tonic.rs", &pkg);

            bad_tonic_file.push(format!("{}.tonic.rs", &pkg));
            good_tonic_file.push(format!("{}_tonic.rs", &pkg));

            debug!(
                "prost file: {} and tonic file: {}",
                &prost_file.to_str().unwrap(),
                &bad_tonic_file.to_str().unwrap()
            );

            self.os_manager()
                .rename_file(&bad_tonic_file, &good_tonic_file)?;

            self.os_manager().find_replace(
                &prost_file,
                RUST_INCLUDE_MACRO.replace(INCLUDE_MACRO_PLACEHOLDER, bad_tonic_filename.as_str()),
                "".to_string(),
            )?;

            self.os_manager().insert_in_position(
                &good_tonic_file,
                0,
                TONIC_USE_SUPER_DIRECTIVE.replace(INCLUDE_MACRO_PLACEHOLDER, &pkg),
            )?;

            let module_file_content: String = format!(
                "pub mod {};\npub mod {};",
                &pkg,
                pkg.to_string() + &String::from("_tonic")
            );

            module_path.push(RUST_STANDARD_MODULE_FILENAME);

            self.os_manager()
                .write_new_file(&module_path, module_file_content)?;
        }

        Ok(())
    }
}
