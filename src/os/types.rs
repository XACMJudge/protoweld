//! Operating system manager trait and platform-specific implementations.

use std::path::PathBuf;

/// Trait for platform-specific system operations.
///
/// This trait abstracts away platform differences for file operations,
/// command execution, and text manipulation. Implementations provide
/// platform-specific behavior while maintaining a consistent interface.
pub trait OSManager {
    /// Executes a command-line program with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute (e.g., "protoc", "go")
    /// * `arguments` - Vector of command-line arguments
    /// * `dependency` - If `true`, this is a dependency check command (shorter timeout)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Command executed successfully
    /// * `Err(String)` - Error message if command fails or times out
    ///
    /// # Behavior
    ///
    /// - Dependency checks use a shorter timeout (1 second)
    /// - Non-dependency commands use a longer timeout (5 seconds)
    /// - Commands that timeout are killed if they're dependency checks
    /// - Stderr is captured and returned in error messages
    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
        dependency: bool,
    ) -> Result<(), String>;

    /// Searches for a string pattern in a file and returns the content and position.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file to search
    /// * `search` - String pattern to search for
    ///
    /// # Returns
    ///
    /// * `Ok((String, usize))` - Tuple of (file content, byte position of first match)
    /// * `Err(String)` - Error message if file cannot be read
    ///
    /// # Note
    ///
    /// If the pattern is not found, the position will be `usize::MAX`.
    fn grep(&self, file: PathBuf, search: &'static str) -> Result<(String, usize), String>;

    /// Renames or moves a file from one path to another.
    ///
    /// # Arguments
    ///
    /// * `old` - Current file path
    /// * `nw` - New file path
    ///
    /// # Returns
    ///
    /// * `Ok(())` - File renamed successfully
    /// * `Err(String)` - Error message if rename operation fails
    fn rename_file(&self, old: &PathBuf, nw: &PathBuf) -> Result<(), String>;

    /// Replaces all occurrences of a pattern with a replacement string in a file.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file to modify
    /// * `pattern` - String pattern to find and replace
    /// * `replace` - Replacement string
    ///
    /// # Returns
    ///
    /// * `Ok(())` - File modified successfully
    /// * `Err(String)` - Error message if file operations fail
    fn find_replace(&self, file: &PathBuf, pattern: String, replace: String) -> Result<(), String>;

    /// Inserts text at a specific byte position in a file.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file to modify
    /// * `position` - Byte position where text should be inserted
    /// * `text` - Text to insert
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Text inserted successfully
    /// * `Err(String)` - Error message if position is out of bounds or file operations fail
    ///
    /// # Panics
    ///
    /// This function will return an error if `position` exceeds the file length.
    fn insert_in_position(&self, file: &PathBuf, position: usize, text: String) -> Result<(), String>;

    /// Creates a new file with the specified content.
    ///
    /// # Arguments
    ///
    /// * `file` - Path where the new file should be created
    /// * `text` - Content to write to the file
    ///
    /// # Returns
    ///
    /// * `Ok(())` - File created successfully
    /// * `Err(String)` - Error message if file creation or writing fails
    fn write_new_file(&self, file: &PathBuf, text: String) -> Result<(), String>;
}

/// Unix-like system manager implementation (Linux, macOS, etc.)
pub struct UnixManager;

/// Windows system manager implementation (not yet implemented), actually idk if this is needed.
pub struct WindowsManager;
