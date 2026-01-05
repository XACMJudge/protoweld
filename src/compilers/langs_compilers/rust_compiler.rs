//! Rust language compiler implementation with post-processing.
//!
//! This compiler generates Rust code using Prost for message types and Tonic
//! for gRPC services. It includes special post-processing to organize generated
//! files into proper Rust module structures.

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

/// Required dependencies for Rust proto compilation.
///
/// These tools must be installed and available:
/// - `protoc`: Protocol Buffers compiler
/// - `protoc-gen-tonic`: Tonic gRPC code generator
/// - `protoc-gen-prost`: Prost message type generator
static RUST_DEPS: [&'static str; 3] = ["protoc", "protoc-gen-tonic", "protoc-gen-prost"];

/// Version flags for checking each dependency.
///
/// Note: `protoc-gen-tonic` and `protoc-gen-prost` don't have standard version
/// flags, so empty strings are used (they'll be checked during actual compilation).
static RUST_VERSION_FLAGS: [&'static str; 3] = ["--version", "", ""];

/// Protoc flag for generating Prost message types.
static RUST_MESSAGES_OUT_ARGUMENT: &'static str = "--prost_out";

/// Protoc flag for generating Tonic gRPC service code.
static RUST_GRPC_OUT_ARGUMENT: &'static str = "--tonic_out";

/// Placeholder used in generated code for package names.
static INCLUDE_MACRO_PLACEHOLDER: &'static str = "package_tonic";

/// Include macro pattern that needs to be removed from Prost-generated files.
static RUST_INCLUDE_MACRO: &'static str = "include!(\"package_tonic\");";

/// Use directive that needs to be added to Tonic-generated files.
static TONIC_USE_SUPER_DIRECTIVE: &'static str = "use super::package_tonic::*;";

/// Standard Rust module filename.
static RUST_STANDARD_MODULE_FILENAME: &'static str = "mod.rs";

impl ProtobufCompiler for RustCompiler {
    /// Compiles proto files to Rust code with post-processing.
    ///
    /// This implementation:
    /// 1. Extracts package names from proto files
    /// 2. Compiles proto files using Prost and Tonic
    /// 3. Performs post-processing to organize files into proper Rust modules:
    ///    - Renames Tonic files from `package.tonic.rs` to `package_tonic.rs`
    ///    - Removes include macros from Prost files
    ///    - Adds use directives to Tonic files
    ///    - Creates `mod.rs` files for each package
    ///
    /// # Arguments
    ///
    /// * `project` - Project configuration with Rust-specific settings
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Rust code generated and organized successfully
    /// * `Err(String)` - Error message if compilation or post-processing fails
    ///
    /// # Generated Output Structure
    ///
    /// For each package, creates:
    /// - `package/package.rs` - Prost-generated message types
    /// - `package/package_tonic.rs` - Tonic-generated gRPC services
    /// - `package/mod.rs` - Module declarations
    ///
    /// # Post-Processing Details
    ///
    /// The Rust code generators (Prost and Tonic) produce files that need
    /// organization to work properly in Rust's module system:
    ///
    /// 1. **File Renaming**: Tonic generates files like `package.tonic.rs`,
    ///    but Rust module names can't contain dots, so we rename to `package_tonic.rs`
    ///
    /// 2. **Include Macro Removal**: Prost files include a macro like
    ///    `include!("package.tonic.rs");` which we remove since we're organizing
    ///    modules manually
    ///
    /// 3. **Use Directive Addition**: Tonic files need a `use super::package::*;`
    ///    directive at the top to access the message types from the Prost module
    ///
    /// 4. **Module File Creation**: We create `mod.rs` files that declare both
    ///    the message module and the gRPC module as public
    fn compile_project(&self, project: &crate::parser::types::Project) -> Result<(), String> {
        // Extract all unique package names from proto files
        let packages: HashSet<String> = self.get_packages_set(&project.associated_proto_files)?;

        // Compile proto files using protoc with Prost and Tonic plugins
        self.assemble_compilation(
            project,
            RUST_DEPS.to_vec(),
            RUST_VERSION_FLAGS.to_vec(),
            RUST_MESSAGES_OUT_ARGUMENT,
            RUST_GRPC_OUT_ARGUMENT,
            Option::None,
        )?;

        let base_path: PathBuf = (&project.compiled_proto_folder).into();

        // Post-process each package to organize files into proper Rust modules
        for pkg in packages.iter() {
            // Build paths for Prost and Tonic files
            let mut prost_file: PathBuf = base_path.clone();
            prost_file.push(pkg);
            let mut module_path: PathBuf = prost_file.clone();
            prost_file.push(format!("{}.rs", &pkg));

            // Tonic generates files with .tonic.rs extension, but we need _tonic.rs
            // for proper Rust module naming (dots aren't allowed in module names)
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

            // Rename Tonic file from package.tonic.rs to package_tonic.rs
            self.os_manager()
                .rename_file(&bad_tonic_file, &good_tonic_file)?;

            // Remove the include macro from Prost file
            // Prost generates: include!("package.tonic.rs");
            // We remove it since we're organizing modules manually
            self.os_manager().find_replace(
                &prost_file,
                RUST_INCLUDE_MACRO.replace(INCLUDE_MACRO_PLACEHOLDER, bad_tonic_filename.as_str()),
                "".to_string(),
            )?;

            // Add use directive to Tonic file to import message types
            // This allows the gRPC code to use types from the Prost module
            self.os_manager().insert_in_position(
                &good_tonic_file,
                0,
                TONIC_USE_SUPER_DIRECTIVE.replace(INCLUDE_MACRO_PLACEHOLDER, &pkg),
            )?;

            // Create mod.rs file that declares both modules as public
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
