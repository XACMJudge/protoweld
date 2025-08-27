use std::{fs, path::PathBuf};

use log::error;

use crate::{
    compilers::types::{GoCompiler, ProtobufCompiler},
    os::shared::get_os_manager,
    parser::types::Lang,
};

pub fn get_compiler(
    lang: &Lang,
    base_file_path: &String,
) -> Result<Box<dyn ProtobufCompiler>, &'static str> {
    let platform_result = get_os_manager();

    if let Err(error) = platform_result {
        return Err(error);
    }

    let platform_manager = platform_result.unwrap();
    let base_path_buf = PathBuf::from(base_file_path);
    let canonical_path_result = fs::canonicalize(&base_path_buf);

    if let Err(error) = &canonical_path_result {
        error!("Canonical path failed {}", error);
        return Err("Canonicalize base path failed");
    }

    match lang {
        Lang::GoLang => Ok(Box::new(GoCompiler {
            os_manager: platform_manager,
            input_file_path: canonical_path_result.unwrap(),
        })),
        Lang::DotNet => todo!("To implement"),
        Lang::Rust => todo!("To implement"),
        _ => Err("Protoweld do not support this lang."),
    }
}
