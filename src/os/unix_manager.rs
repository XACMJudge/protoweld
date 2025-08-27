use crate::os::types::{OSManager, UnixManager};

impl OSManager for UnixManager {
    fn ensure_installation(&self, command: &'static str) -> Result<String, String> {
        todo!()
    }
}
