pub trait OSManager {
    fn ensure_installation(
        &self,
        command: &'static str,
        version_flag: &'static str,
    ) -> Result<(), String>;

    fn execute_command(&self, command: &'static str, arguments: &Vec<String>)
    -> Result<(), String>;
}

pub struct UnixManager;
pub struct WindowsManager;
