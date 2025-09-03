use std::{
    process::{Command, Stdio},
    time::Duration,
};

use log::info;
use wait_timeout::ChildExt;

use crate::os::types::{OSManager, UnixManager};

impl OSManager for UnixManager {
    fn execute_command(
        &self,
        command: &'static str,
        arguments: &Vec<String>,
        dependency: bool,
    ) -> Result<(), String> {
        let child_result = Command::new(command)
            .args(arguments)
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .spawn();

        if let Err(err) = child_result {
            info!("Child spawn error: {}", err);
            return Err(err.to_string());
        }

        let mut child = child_result.unwrap();
        let time_out_duration = Duration::from_secs(if dependency { 1 } else { 5 });

        match child.wait_timeout(time_out_duration).unwrap() {
            Some(status) => {
                if status.success() {
                    Ok(())
                } else {
                    if let Some(stderr) = child.stderr.take() {
                        Err(std::io::read_to_string(stderr).unwrap())
                    } else {
                        Err(format!("Dep command exited with status: {}", status))
                    }
                }
            }
            None => {
                // Fine, some plugins just timeout
                child.kill().unwrap();
                child.wait().unwrap();

                if dependency {
                    Err(format!(
                        "{} command timed out after {} seconds.",
                        command,
                        time_out_duration.as_secs()
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}
