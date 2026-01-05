//! Factory function for creating platform-specific OS managers.

use current_platform::CURRENT_PLATFORM;
use log::info;

use crate::os::types::{OSManager, UnixManager};

/// Creates an appropriate OS manager for the current platform.
///
/// This function detects the current operating system and returns the
/// corresponding `OSManager` implementation. Currently supports Unix-like
/// systems (Linux, macOS).
///
/// # Returns
///
/// * `Ok(Box<dyn OSManager>)` - Platform-specific OS manager instance
/// * `Err(&'static str)` - Error if the platform is not supported
///
/// # Supported Platforms
///
/// - Linux (any distribution)
/// - macOS
///
/// # Future Support
///
/// Windows support is planned but not yet implemented.
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
