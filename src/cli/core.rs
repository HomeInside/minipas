use crate::Rule;
use crate::parser::ast::Stmt;
use pest::iterators::Pairs;
use std::path::{Path, PathBuf};
use std::{ffi::OsStr, fs};

pub fn parse_input_file(s: &OsStr) -> Result<PathBuf, &'static str> {
    if Path::new(s).exists() {
        Ok(s.into())
    } else {
        Err("input file does not exist")
    }
}

pub fn parse_output_file(s: &OsStr) -> Result<PathBuf, &'static str> {
    if Path::new(s).exists() {
        Err("there is already a file with that name")
    } else {
        Ok(s.into())
    }
}

/// Lee archivo el archivo y retorna el String
///
pub fn read_source(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("No se pudo leer el archivo '{}'", path.display()))
}

pub fn save_pairs_as_text(pairs: &Pairs<Rule>, output: &PathBuf) {
    std::fs::write(output, format!("{:#?}", pairs)).expect("No se pudo escribir archivo .mpp");
}

pub fn save_ast_as_text(code: &[Stmt], output: &PathBuf) {
    std::fs::write(output, format!("{:#?}", code)).expect("No se pudo escribir archivo .mpa");
}
