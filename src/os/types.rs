pub trait OSManager {
    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
        dependency: bool,
    ) -> Result<(), String>;
}

pub struct UnixManager;
pub struct WindowsManager;
