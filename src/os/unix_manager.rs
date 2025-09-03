use std::{
    io::Stdout,
    process::{Command, Stdio},
    ptr::null,
};

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

    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
    ) -> Result<(), String> {
        let command_result = Command::new(command)
            .args(arguments)
            .stdout(Stdio::null())
            .stdin(Stdio::null())
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
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }
}
