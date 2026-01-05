//! Base compiler trait and shared compilation logic.

use std::{collections::HashSet, path::PathBuf};

use crate::{os::types::OSManager, parser::types::Project};

/// Keyword used to identify package declarations in proto files.
static PACKAGE_KEYWORD: &'static str = "package";

/// Character used to terminate package declarations in proto files.
static SEMICOLON: &'static str = ";";

/// Trait providing access to compiler properties and dependencies.
///
/// This trait provides access to the OS manager and input file path,
/// which are needed by all compiler implementations.
pub trait CompilerProperties {
    /// Returns a reference to the OS manager for system operations.
    fn os_manager(&self) -> &Box<dyn OSManager>;

    /// Returns the canonicalized path to the input configuration file.
    fn input_file_path(&self) -> &PathBuf;
}

/// Main trait for Protocol Buffer compilers.
///
/// This trait defines the interface for compiling `.proto` files into
/// language-specific code. Implementations provide language-specific
/// compilation logic while sharing common functionality like dependency
/// checking and package extraction.
pub trait ProtobufCompiler: CompilerProperties {
    /// Compiles all proto files for a project into the target language.
    ///
    /// This is the main entry point for compilation. It orchestrates
    /// dependency checking, command assembly, and execution.
    ///
    /// # Arguments
    ///
    /// * `project` - Project configuration containing proto files and options
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Compilation succeeded
    /// * `Err(String)` - Error message if compilation fails
    fn compile_project(&self, project: &Project) -> Result<(), String>;
    /// Extracts package names from a list of proto files.
    ///
    /// This function searches each proto file for the `package` declaration
    /// and extracts the package name. It's used primarily by the Rust compiler
    /// to organize generated files into proper module structures.
    ///
    /// # Arguments
    ///
    /// * `protos` - Vector of paths to proto files
    ///
    /// # Returns
    ///
    /// * `Ok(HashSet<String>)` - Set of unique package names found
    /// * `Err(String)` - Error if a proto file is missing a package declaration
    ///
    /// # Process
    ///
    /// 1. Searches each proto file for the "package" keyword
    /// 2. Extracts the package name (text between "package" and ";")
    /// 3. Collects unique package names into a HashSet
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A proto file cannot be read
    /// - A proto file doesn't contain a package declaration
    /// - The package declaration is malformed
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
    /// Verifies that all required dependencies are installed and accessible.
    ///
    /// This function checks each dependency by running it with a version flag.
    /// Dependencies are checked with a short timeout since version checks should be fast.
    ///
    /// # Arguments
    ///
    /// * `deps` - Vector of dependency command names (e.g., "protoc", "go")
    /// * `version_flags` - Vector of version flags for each dependency (e.g., "--version", "version")
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All dependencies are installed and accessible
    /// * `Err(Vec<&'static str>)` - List of dependencies that failed the check
    ///
    /// # Note
    ///
    /// The vectors must have the same length. Each dependency is checked by running
    /// `dependency version_flag` and checking if it succeeds.
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

    /// Assembles and executes the protoc compilation command.
    ///
    /// This function orchestrates the entire compilation process:
    /// 1. Checks that all dependencies are installed
    /// 2. Assembles the protoc command with appropriate flags
    /// 3. Handles plugin configuration if needed
    /// 4. Executes the compilation
    ///
    /// # Arguments
    ///
    /// * `project` - Project configuration with proto files and options
    /// * `compiler_deps` - List of required dependencies for this language
    /// * `compiler_version_flags` - Version flags for dependency checking
    /// * `compiler_out_flag` - Protoc flag for message output (e.g., "--go_out", "--csharp_out")
    /// * `compiler_out_grpc_flag` - Protoc flag for gRPC output (e.g., "--go-grpc_out", "--grpc_out")
    /// * `compiler_plugin` - Optional plugin name if a custom gRPC plugin is needed
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Compilation succeeded
    /// * `Err(String)` - Error message if compilation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required dependencies are missing
    /// - Output flags are specified in compile_options (they're handled automatically)
    /// - Plugin is required but plugin_path is not provided
    /// - Protoc command execution fails
    fn assemble_compilation(
        &self,
        project: &Project,
        compiler_deps: Vec<&'static str>,
        compiler_version_flags: Vec<&'static str>,
        compiler_out_flag: &'static str,
        compiler_out_grpc_flag: &'static str,
        compiler_plugin: Option<&'static str>,
    ) -> Result<(), String> {
        // Check that all required dependencies are installed
        if let Err(failed_dependencies) =
            self.ensure_dependencies(&compiler_deps, &compiler_version_flags)
        {
            let mut error_msg: String =
                String::from("Failed to check installation of the following dependencies: ");
            error_msg.push_str(failed_dependencies.join(",").as_str());

            return Err(error_msg);
        }

        // Build the protoc command arguments
        let mut command_args: Vec<String> = Vec::new();

        // Add custom compile options from the project configuration
        for (key, value) in project.compile_options.iter() {
            // Prevent users from specifying output flags manually (we handle them)
            if key == compiler_out_flag || key == compiler_out_grpc_flag {
                let err = format!(
                    "The argument {} do not must appear. Protoweld handle this using compiled_proto_folder option",
                    key
                );
                return Err(err);
            }

            // Format flags: empty value means flag-only (e.g., "--include_imports"),
            // non-empty means key=value (e.g., "-I=entities")
            let flag = match value == "" {
                true => format!("{}", key),
                false => format!("{}={}", key, value),
            };
            command_args.push(flag);
        }

        // Add output flags for messages and gRPC services
        command_args.push(format!(
            "{}={}",
            &compiler_out_flag, &project.compiled_proto_folder,
        ));

        command_args.push(format!(
            "{}={}",
            &compiler_out_grpc_flag, &project.compiled_proto_folder,
        ));

        // Add plugin configuration if needed (e.g., for .NET)
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

        // Add all proto files to compile
        for proto in project.associated_proto_files.iter() {
            command_args.push(String::from(proto));
        }

        // Execute the protoc command
        if let Err(err) = self
            .os_manager()
            .execute_command("protoc", &command_args, false)
        {
            return Err(err);
        }
        Ok(())
    }
}
