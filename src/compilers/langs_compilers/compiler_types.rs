use std::path::PathBuf;

use crate::{
    compilers::protobuf_compiler::ProtobufCompiler, os::types::OSManager, parser::types::Lang,
};

pub struct CompilerParams {
    pub os_manager: Box<dyn OSManager>,
    pub input_file_path: PathBuf,
    pub lang: Lang,
}

// INFO: We need separate structs for langs because their compilation may vary according the lang
// itself. (Ej. Rust needs two commands)

pub struct GoCompiler {
    pub params: CompilerParams,
}
pub struct DotNetCompiler {
    pub params: CompilerParams,
}
pub struct RustCompiler {
    pub params: CompilerParams,
}

impl From<CompilerParams> for Box<dyn ProtobufCompiler> {
    fn from(params: CompilerParams) -> Self {
        match params.lang {
            Lang::GoLang => Box::new(GoCompiler { params }),
            Lang::DotNet => Box::new(DotNetCompiler { params }),
            Lang::Rust => Box::new(RustCompiler { params }),
        }
    }
}
