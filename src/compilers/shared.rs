use crate::{
    compilers::types::{GoCompiler, ProtobufCompiler},
    os::shared::get_os_manager,
    parser::types::Lang,
};

pub fn get_compiler(lang: &Lang) -> Result<Box<dyn ProtobufCompiler>, &'static str> {
    let platform_result = get_os_manager();

    if let Err(error) = platform_result {
        return Err(error);
    }

    let platform_manager = platform_result.unwrap();

    match lang {
        Lang::GoLang => Ok(Box::new(GoCompiler {
            os_manager: platform_manager,
        })),
        Lang::DotNet => todo!("To implement"),
        Lang::Rust => todo!("To implement"),
        _ => Err("Protoweld do not support this lang."),
    }
}
