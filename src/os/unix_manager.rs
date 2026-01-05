//! Unix-like system implementation of the OSManager trait.
//!
//! This module provides Unix-specific implementations for file operations,
//! command execution, and text manipulation. It handles Linux and macOS systems.

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use log::info;
use wait_timeout::ChildExt;

use crate::os::types::{OSManager, UnixManager};

/// Timeout duration in seconds for dependency check commands (e.g., `protoc --version`).
///
/// Dependency checks should be fast, so a short timeout is used.
static DEPENDENCY_COMMAND_TIMEOUT: u64 = 1;

/// Timeout duration in seconds for non-dependency commands (e.g., actual proto compilation).
///
/// Compilation commands may take longer, so a longer timeout is used.
static NON_DEPENDENCY_COMMAND_TIMEOUT: u64 = 5;

impl OSManager for UnixManager {
    /// Searches for a pattern in a file and returns the content with match position.
    ///
    /// This implementation reads the entire file into memory and searches for
    /// the pattern using string matching. The position is returned as a byte offset.
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file to search
    /// * `search` - String pattern to find
    ///
    /// # Returns
    ///
    /// * `Ok((String, usize))` - Tuple of (file content, byte position of first match)
    /// * `Err(String)` - Error if file cannot be read
    ///
    /// # Note
    ///
    /// Returns `usize::MAX` as the position if the pattern is not found.
    fn grep(&self, file: PathBuf, search: &'static str) -> Result<(String, usize), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let text = content.unwrap();
        Ok((
            text.clone(),
            match text.find(&search) {
                Some(index) => index.into(),
                None => usize::MAX,
            },
        ))
    }

    /// Executes a command with timeout handling and error capture.
    ///
    /// This implementation spawns a child process, captures stderr, and applies
    /// a timeout based on whether this is a dependency check or a full compilation.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute
    /// * `arguments` - Command-line arguments
    /// * `dependency` - If true, uses shorter timeout and fails on timeout
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Command succeeded
    /// * `Err(String)` - Error message with stderr or timeout information
    ///
    /// # Behavior
    ///
    /// - Dependency checks: 1 second timeout, failure on timeout
    /// - Compilation commands: 5 second timeout, success on timeout (some plugins hang)
    /// - Stderr is captured and returned in error messages
    /// - Commands that timeout are killed
    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
        dependency: bool,
    ) -> Result<(), String> {
        let child_result = Command::new(command)
            .args(arguments)
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .spawn();

        if let Err(err) = child_result {
            info!("Child spawn error: {}", err);
            return Err(err.to_string());
        }

        let mut child = child_result.unwrap();
        let time_out_duration = Duration::from_secs(if dependency {
            DEPENDENCY_COMMAND_TIMEOUT
        } else {
            NON_DEPENDENCY_COMMAND_TIMEOUT
        });

        match child.wait_timeout(time_out_duration).unwrap() {
            Some(status) => {
                if status.success() {
                    Ok(())
                } else {
                    if let Some(stderr) = child.stderr.take() {
                        Err(std::io::read_to_string(stderr).unwrap())
                    } else {
                        Err(format!("Dep command exited with status: {}", status))
                    }
                }
            }
            None => {
                // Fine, some plugins just timeout
                child.kill().unwrap();
                child.wait().unwrap();

                if dependency {
                    Err(format!(
                        "{} command timed out after {} seconds.",
                        command,
                        time_out_duration.as_secs()
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Renames a file using the filesystem rename operation.
    ///
    /// This is an atomic operation on Unix systems when both paths are on the same filesystem.
    fn rename_file(&self, old: &PathBuf, nw: &PathBuf) -> Result<(), String> {
        let result = fs::rename(old, nw);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    /// Replaces all occurrences of a pattern in a file.
    ///
    /// Reads the entire file, performs string replacement, and writes it back.
    /// This operation is not atomic - if writing fails, the file may be corrupted.
    fn find_replace(&self, file: &PathBuf, pattern: String, replace: String) -> Result<(), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let modified: String = content.unwrap().replace(&pattern, &replace);
        let result = fs::write(file, modified);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    /// Inserts text at a specific byte position in a file.
    ///
    /// This operation reads the file, inserts the text at the specified position,
    /// and writes it back. The position is a byte offset, not a character offset.
    ///
    /// # Errors
    ///
    /// Returns an error if the position exceeds the file length.
    fn insert_in_position(
        &self,
        file: &PathBuf,
        position: usize,
        text: String,
    ) -> Result<(), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let mut file_str: String = content.unwrap();
        if position <= file_str.len() {
            file_str.insert_str(position, &text);
        } else {
            return Err(String::from(format!(
                "Position out of bounds. File lenght {} characters",
                file_str.len()
            )));
        }

        let result = fs::write(file, file_str);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a new file and writes content to it.
    ///
    /// This will overwrite any existing file at the specified path.
    fn write_new_file(&self, file: &PathBuf, text: String) -> Result<(), String> {
        let file_result = fs::File::create(file);
        if let Err(e) = file_result {
            return Err(e.to_string());
        }

        match file_result.unwrap().write_all(text.as_bytes()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }
}
