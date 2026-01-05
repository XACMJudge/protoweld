# Protoweld

**Protoweld** is a powerful command-line tool designed to automate the compilation of Protocol Buffer (`.proto`) files into language-specific code for multiple projects. It simplifies the process of managing and generating protobuf code across different programming languages and projects, making it ideal for microservices architectures and multi-language codebases.

## Features

- 🚀 **Multi-language Support**: Compile `.proto` files to Go, .NET (C#), and Rust
- 📁 **Multi-project Management**: Handle multiple projects with different configurations in a single YAML file
- 🔧 **Automatic Dependency Checking**: Verifies required tools are installed before compilation
- 🎯 **Flexible Configuration**: Customize compilation options per project
- 🔌 **Plugin Support**: Configure custom gRPC plugins (e.g., for .NET)
- 📦 **Smart File Organization**: Automatically organizes generated code into specified folders
- 🛠️ **Rust-specific Optimizations**: Special handling for Rust projects with Tonic and Prost

## Installation

### Standard Installation

Protoweld is available on [crates.io](https://crates.io/crates/protoweld) and can be installed using Cargo:

```bash
cargo install protoweld
```

After installation, the `protoweld` binary will be available in your `$HOME/.cargo/bin` directory (make sure it's in your `PATH`).

### Prerequisites

Protoweld requires the following tools to be installed on your system:

- **Rust and Cargo** - Required for installation via `cargo install`. [Install Rust](https://www.rust-lang.org/tools/install)
- **Protocol Buffers Compiler** (`protoc`) - Required for compiling proto files. [Installation Guide](https://grpc.io/docs/protoc-installation/)

### Building from Source

If you prefer to build from source or want to use the latest development version:

1. Clone the repository:
```bash
git clone https://github.com/XACMJudge/protoweld.git
cd protoweld
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/protoweld`

### Installing Dependencies for Each Language

#### Go
```bash
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

#### .NET
- Install the .NET SDK
- Install gRPC Tools NuGet package (the plugin path will be in your NuGet packages folder)

#### Rust
```bash
cargo install protoc-gen-tonic
cargo install protoc-gen-prost
```

## Configuration

Protoweld uses a YAML configuration file to define projects and their compilation settings. The configuration file structure is as follows:

```yaml
active_projects:
  - path: <project-name>
    lang: <GoLang|DotNet|Rust>
    associated_proto_files:
      - <path-to-proto-file-1>
      - <path-to-proto-file-2>
    compiled_proto_folder: "<output-folder-path>"
    plugin_path: "<optional-plugin-path>"
    compile_options:
      "<option-key>": "<option-value>"
      "<option-key>": ""
```

### Configuration Fields

#### Required Fields

- **`path`** (string): A unique identifier for the project
- **`lang`** (string): Target programming language. Must be one of: `GoLang`, `DotNet`, or `Rust`
- **`associated_proto_files`** (array of strings): List of paths to `.proto` files to compile
- **`compiled_proto_folder`** (string): Output directory where generated code will be placed

#### Optional Fields

- **`plugin_path`** (string): Path to a custom gRPC plugin (required for .NET projects)
- **`compile_options`** (map): Additional compilation options passed to `protoc`

### Compile Options

The `compile_options` field allows you to pass custom flags to the Protocol Buffers compiler. Common options include:

- **`-I`** or **`--proto_path`**: Specify import paths for proto files
- **`--descriptor_set_out`**: Generate a descriptor set file
- **`--include_imports`**: Include all imported files in the descriptor set
- **`--experimental_allow_proto3_optional`**: Enable proto3 optional fields

**Note**: The output flags (`--go_out`, `--csharp_out`, `--prost_out`, etc.) are automatically handled by Protoweld and should not be specified in `compile_options`.

## Usage

Run Protoweld with the path to your configuration file:

```bash
protoweld -f <path-to-config.yaml>
```

Or using the short form:

```bash
protoweld -f config.yaml
```

### Example

```bash
protoweld -f input/example.yaml
```

## Example Configuration

Here's a complete example configuration file (`input/example.yaml`):

```yaml
active_projects:
  - path: database-server
    associated_proto_files:
      - ./entities/protos/database-server/operations.proto
    compiled_proto_folder: "./database-server/"
    lang: GoLang
    compile_options:
      "-I": entities

  - path: security
    lang: DotNet
    associated_proto_files:
      - ./entities/protos/security/users.proto
      - ./entities/protos/security/auth.proto
      - ./entities/protos/schemas/security.proto
      - ./entities/protos/database-server/operations.proto
    compiled_proto_folder: "./security/Protos"
    plugin_path: /home/user/.nuget/packages/grpc.tools/2.72.0/tools/linux_x64/grpc_csharp_plugin
    compile_options:
      "-I": entities
      "--descriptor_set_out": ./security/descriptors.pb
      "--include_imports": ""
      "--experimental_allow_proto3_optional": ""

  - path: judge-server
    lang: Rust
    associated_proto_files:
      - ./entities/protos/database-server/operations.proto
    compiled_proto_folder: "./judge-server/protos"
    compile_options:
      "-I": entities
```

## Supported Languages

### Go (GoLang)

Protoweld compiles `.proto` files to Go code using:
- `protoc-gen-go` for message types
- `protoc-gen-go-grpc` for gRPC service definitions

**Generated Output**: Go source files in the specified `compiled_proto_folder`

### .NET (DotNet)

Protoweld compiles `.proto` files to C# code using:
- `protoc` with `--csharp_out` for message types
- `grpc_csharp_plugin` for gRPC service definitions

**Requirements**: 
- Must specify `plugin_path` pointing to the `grpc_csharp_plugin` executable
- Typically located in: `~/.nuget/packages/grpc.tools/<version>/tools/<platform>/grpc_csharp_plugin`

**Generated Output**: C# source files in the specified `compiled_proto_folder`

### Rust

Protoweld compiles `.proto` files to Rust code using:
- `protoc-gen-prost` for message types (Prost)
- `protoc-gen-tonic` for gRPC service definitions (Tonic)

**Special Features**:
- Automatically organizes generated files into proper Rust module structure
- Creates `mod.rs` files for each package
- Handles file renaming and module imports automatically

**Generated Output**: 
- Rust source files organized by package in the specified `compiled_proto_folder`
- Each package gets its own module with proper `mod.rs` structure

## How It Works

1. **Parsing**: Protoweld reads and parses the YAML configuration file
2. **Validation**: For each project, it validates that:
   - Required dependencies are installed
   - Proto files exist and contain valid `package` declarations
   - Output directories can be created
3. **Compilation**: For each project, it:
   - Assembles the appropriate `protoc` command with language-specific flags
   - Executes the compilation
   - Handles language-specific post-processing (especially for Rust)
4. **Output**: Generated code is placed in the specified `compiled_proto_folder` for each project

### Why reorganize the Rust generated files?

Protoweld applies extensive transformations to the Rust-generated files mainly to ensure a smooth developer experience with tools like **rust-analyzer** and to make the generated code ready for ongoing development, not just for one-off code generation.

By default, when the gRPC plugins (`protoc-gen-prost` for message types and `protoc-gen-tonic` for services) generate Rust code from `.proto` files, they output raw `.rs` files that closely mirror the original proto package and file structure. However, these raw files often lack a module structure compatible with how Rust and its tooling (such as rust-analyzer and cargo) expect code to be organized. For example, there may be missing `mod.rs` files, incorrect or missing module imports, and file names that don't align with idiomatic Rust conventions.

**Why does Protoweld reorganize these files?**

- **Rust Tooling Compatibility**: Tools like rust-analyzer, and even cargo itself, expect Rust code to be organized into modules using either `mod.rs` files or the new [directory-as-module conventions](https://doc.rust-lang.org/1.30.0/src/std/lib.rs.html#modules). The flat file structure output by the plugins does not provide this, making IDE navigation, auto-completion, and refactoring much less effective.
- **Correct Imports and Modules**: The generated files may refer to each other using flat paths, or miss Rust-specific `mod` declarations entirely, which can break module imports or make it difficult to include generated code in your own crate.
- **Development-Ready Code**: Instead of just dumping generated code, Protoweld restructures it so that you can immediately use the generated modules in your own Rust project without manual adjustments. This includes creating `mod.rs` files, renaming files to valid Rust identifiers, and setting up imports and visibility declarations.

**Summary**: Protoweld's file transformations are primarily driven by the needs of real-world Rust development, ensuring that generated Protobuf code is both compiler- and IDE-friendly, resulting in a much smoother workflow for developers.

## Project Structure

```no-run
protoweld/
├── Cargo.toml              # Rust project configuration
├── README.md               # This file
├── LICENSE                 # License file
├── input/                  # Example configuration files
│   └── example.yaml       # Example YAML configuration
└── src/                    # Source code
    ├── main.rs            # Entry point
    ├── lib.rs             # Library root
    ├── types/             # Type definitions
    │   ├── mod.rs
    │   └── cli.rs         # CLI argument parsing
    ├── parser/            # YAML configuration parser
    │   ├── mod.rs
    │   ├── protoweld_parser.rs
    │   └── types.rs       # Parser types and structures
    ├── executor/          # Code generation executor
    │   ├── mod.rs
    │   └── protoweld_executor.rs
    ├── compilers/         # Language-specific compilers
    │   ├── mod.rs
    │   ├── protobuf_compiler.rs  # Base compiler trait
    │   ├── shared.rs      # Compiler factory
    │   └── langs_compilers/
    │       ├── mod.rs
    │       ├── compiler_types.rs
    │       ├── go_compiler.rs
    │       ├── dotnet_compiler.rs
    │       └── rust_compiler.rs
    └── os/                # OS abstraction layer
        ├── mod.rs
        ├── types.rs       # OS manager trait
        ├── shared.rs      # OS manager factory
        └── unix_manager.rs
```

## Error Handling

Protoweld provides clear error messages for common issues:

- **Missing dependencies**: Lists which required tools are not installed
- **Invalid proto files**: Reports proto files missing `package` declarations
- **Configuration errors**: Validates YAML structure and required fields
- **Compilation failures**: Passes through `protoc` error messages

## Troubleshooting

### "Failed to check installation of dependencies"

Ensure all required tools for your target language are installed and available in your `PATH`:
- For Go: `protoc-gen-go` and `protoc-gen-go-grpc`
- For .NET: `dotnet` and `protoc`
- For Rust: `protoc-gen-tonic` and `protoc-gen-prost`

### "Package keyword missing in [proto-file]"

Your `.proto` file must include a `package` declaration. Example:
```protobuf
syntax = "proto3";
package mypackage;

message MyMessage {
  // ...
}
```

This is specially required for Rust projects.

### "The plugin [plugin-name] must have a path in plugin_path option"

For .NET projects, you must specify the `plugin_path` to the `grpc_csharp_plugin` executable. Find it in your NuGet packages folder or install gRPC Tools.

### Path Issues

- Use relative paths from the configuration file's location
- Ensure proto file paths are correct and files exist
- Output folders will be created if they don't exist

## Future Plans

Protoweld is actively being developed with the following features planned:

### High Priority

- **Wildcard Support in File Lists**: Support glob patterns in `associated_proto_files` to automatically discover and include proto files (e.g., `./protos/**/*.proto` or `./entities/**/*.proto`)
- **Windows Platform Support**: Investigate whether Windows-specific code is needed or if Rust's high-level cross-platform APIs are sufficient. Currently, Protoweld uses Unix-specific implementations, but Rust's standard library may provide adequate cross-platform abstractions
- **Additional Language Support**: Expand support to more programming languages, including:
  - Python (using `grpcio-tools`)
  - Java (using `protoc-gen-java` and `protoc-gen-grpc-java`)
  - TypeScript/JavaScript (using `protoc-gen-ts` and `@grpc/proto-loader`)
  - C++ (using `protoc-gen-cpp`)
  - PHP (using `protoc-gen-php`)
  - Ruby (using `grpc-tools`)
  - Swift (using `protoc-gen-swift`)
- **Automatic Plugin Discovery**: Automatically search for and locate gRPC plugins in common installation locations (e.g., `$HOME/.cargo/bin`, `$HOME/.local/bin`, NuGet packages, npm global packages) to reduce manual configuration

### Additional Proposals

- **Incremental Compilation**: Only recompile proto files that have changed since the last run, improving performance for large projects
- **Watch Mode**: Automatically recompile proto files when source files change, similar to `cargo watch` or file system watchers
- **Enhanced Error Messages**: Provide actionable error messages with suggestions for fixing common issues (e.g., "Did you mean to install protoc-gen-go?")
- **Configuration Validation**: Validate configuration files before compilation starts, catching errors early
- **Parallel Compilation**: Compile multiple projects in parallel to reduce total build time
- **Dry-Run Mode**: Preview what would be compiled without actually executing the compilation
- **Verbose/Debug Output**: Add detailed logging modes to help debug compilation issues
- **Configuration Templates**: Generate example configuration files for common project structures
- **Proto Dependency Graph**: Analyze and visualize dependencies between proto files
- **Version Pinning**: Support pinning specific versions of protoc and language plugins
- **CI/CD Integration**: Add features specifically for continuous integration environments (e.g., caching, artifact management)
- **Configuration Inheritance**: Allow projects to inherit common settings from a base configuration
- **Multi-version Support**: Support compiling the same proto files to multiple language versions (e.g., Go 1.18 and 1.19)

### Contributing

Contributions are welcome! If you'd like to help implement any of these features or have other ideas, please feel free to submit a Pull Request or open an issue to discuss.

## License

See the [LICENSE](https://github.com/XACMJudge/protoweld/blob/main/LICENSE) file for details.

## Acknowledgments

Protoweld is built to simplify Protocol Buffer compilation workflows in multi-language projects, particularly useful for microservices architectures and distributed systems.
