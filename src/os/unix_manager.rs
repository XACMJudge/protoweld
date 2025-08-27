use std::process::{Command, Stdio};

use log::info;

use crate::os::types::{OSManager, UnixManager};

impl OSManager for UnixManager {
    fn ensure_installation(
        &self,
        command: &'static str,
        version_flag: &'static str,
    ) -> Result<(), String> {
        let command_result = Command::new(command)
            .arg(version_flag)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();

        if let Err(error) = command_result {
            let error_str: String = error.to_string();
            info!("Command {} exited with {}", command, &error_str);
            return Err(error_str);
        }

        let output = command_result.unwrap();

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from("Not installed"))
        }
    }
}
