use std::path::{Path, PathBuf};
use std::{ffi::OsStr, fs};

pub fn parse_input_file(s: &OsStr) -> Result<PathBuf, &'static str> {
    match Path::new(s).exists() {
        true => Ok(s.into()),
        false => Err("input file does not exist"),
    }
}

pub fn parse_output_file(s: &OsStr) -> Result<PathBuf, &'static str> {
    match Path::new(s).exists() {
        true => Err("there is already a file with that name"),
        false => Ok(s.into()),
    }
}

/// Lee archivo a String
pub fn read_source(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("No se pudo leer el archivo '{}'", path.display()))
}
