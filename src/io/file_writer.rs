use std::{collections::HashMap, fs, path::PathBuf};

use crate::types::custom_error::Result;

pub struct FileWriter;

static CONFIG_PATH: &str = ".protoweld/";

impl FileWriter {
    pub fn write_json(entries: &HashMap<&PathBuf, String>) -> Result<()> {

        fs::create_dir(CONFIG_PATH)?;

        let json_string = serde_json::to_string(entries)?;
        let mut file_path_str: String = String::from(CONFIG_PATH);
        file_path_str.push_str("sha.json");
        fs::write(file_path_str, json_string)?;
        Ok(())
    }
}
