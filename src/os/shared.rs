use current_platform::CURRENT_PLATFORM;
use log::info;

use crate::os::types::OSManager;

pub fn get_os_manager() -> Result<Box<dyn OSManager>, &'static str> {
    info!(
        "[PROTOWELD OS MANAGER] Current platform: {}",
        CURRENT_PLATFORM
    );
    match CURRENT_PLATFORM {
        _ => Err("Platform do not supported"),
    }
}
