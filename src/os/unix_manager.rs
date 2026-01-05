use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use log::info;
use wait_timeout::ChildExt;

use crate::os::types::{OSManager, UnixManager};

static DEPENDENCY_COMMAND_TIMEOUT: u64 = 1;
static NON_DEPENDENCY_COMMAND_TIMEOUT: u64 = 5;

impl OSManager for UnixManager {
    fn grep(&self, file: PathBuf, search: &'static str) -> Result<(String, usize), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let text = content.unwrap();
        Ok((
            text.clone(),
            match text.find(&search) {
                Some(index) => index.into(),
                None => usize::MAX,
            },
        ))
    }

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
        let time_out_duration = Duration::from_secs(if dependency {
            DEPENDENCY_COMMAND_TIMEOUT
        } else {
            NON_DEPENDENCY_COMMAND_TIMEOUT
        });

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

    fn rename_file(&self, old: &PathBuf, nw: &PathBuf) -> Result<(), String> {
        let result = fs::rename(old, nw);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    fn find_replace(&self, file: &PathBuf, pattern: String, replace: String) -> Result<(), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let modified: String = content.unwrap().replace(&pattern, &replace);
        let result = fs::write(file, modified);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    fn insert_in_position(
        &self,
        file: &PathBuf,
        position: usize,
        text: String,
    ) -> Result<(), String> {
        let content = fs::read_to_string(file);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let mut file_str: String = content.unwrap();
        if position <= file_str.len() {
            file_str.insert_str(position, &text);
        } else {
            return Err(String::from(format!(
                "Position out of bounds. File lenght {} characters",
                file_str.len()
            )));
        }

        let result = fs::write(file, file_str);
        if let Err(e) = result {
            Err(e.to_string())
        } else {
            Ok(())
        }
    }

    fn write_new_file(&self, file: &PathBuf, text: String) -> Result<(), String> {
        let file_result = fs::File::create(file);
        if let Err(e) = file_result {
            return Err(e.to_string());
        }

        match file_result.unwrap().write_all(text.as_bytes()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }
}
