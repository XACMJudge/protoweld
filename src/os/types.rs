use std::path::PathBuf;

pub trait OSManager {
    /// Execute a CLI command
    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
        dependency: bool,
    ) -> Result<(), String>;

    // Gets the first match
    fn grep(&self, file: PathBuf, search: &'static str) -> Result<(String, usize), String>;

    fn rename_file(&self, old: &PathBuf, nw: &PathBuf) -> Result<(), String>;
    fn find_replace(&self, file: &PathBuf, pattern: String, replace: String) -> Result<(), String>;
    fn insert_in_position(&self, file: &PathBuf, position: usize, text: String) -> Result<(), String>;
    fn write_new_file(&self, file: &PathBuf, text: String) -> Result<(), String>;
}


pub struct UnixManager;
pub struct WindowsManager;
