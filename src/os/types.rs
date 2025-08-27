pub trait OSManager {
    fn ensure_installation(&self,command: &'static str) -> Result<String, String>;
}

pub struct UnixManager;
pub struct WindowsManager;
