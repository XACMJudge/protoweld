use current_platform::CURRENT_PLATFORM;
use log::info;

use crate::os::types::{OSManager, UnixManager};

pub fn get_os_manager() -> Result<Box<dyn OSManager>, &'static str> {
    info!(
        "[PROTOWELD OS MANAGER] Current platform: {}",
        CURRENT_PLATFORM
    );

    let platform_str: String = String::from(CURRENT_PLATFORM);
    if platform_str.contains("linux") {
        return Ok(Box::new(UnixManager));
    }

    Err("Platform do not recognized")
}
