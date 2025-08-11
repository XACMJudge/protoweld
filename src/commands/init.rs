use std::{collections::HashMap, env, ffi::OsStr, fs, path::PathBuf};

use crate::{
    algorithm::hash::hash_sha,
    io::file_writer::FileWriter,
    types::{
        custom_error::{ProtoWeldError, Result},
        init_command::{InitActions, InitCommand},
    },
};
use walkdir::WalkDir;

impl InitActions for InitCommand {
    fn init() -> Result<()> {
        let path = env::current_dir()?;

        let mut hash_paths: Vec<PathBuf> = Vec::new();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("proto"))
            {
                hash_paths.push(entry.path().to_path_buf());
            }
        }
        InitCommand::build_hash(&hash_paths)?;
        Ok(())
    }

    fn build_hash(paths: &Vec<PathBuf>) -> Result<()> {
        let mut entries: HashMap<&PathBuf, String> = HashMap::new();
        println!("Detected protos...");
        for path in paths {
            println!("{}", path.file_name().ok_or(ProtoWeldError)?.display());
            let content = fs::read_to_string(path)?;
            entries.insert(path, hash_sha(content));
        }
        FileWriter::write_json(&entries)?;
        Ok(())
    }
}
