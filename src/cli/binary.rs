use crate::parser::ast::Stmt;
use bincode::{Decode, Encode, config};
use std::path::PathBuf;

#[derive(Encode, Decode, Debug)]
pub struct BinAstFile(Vec<Stmt>);

use pest::iterators::Pairs;

use crate::Rule;

pub fn gen_bincode_from_ast(code: Vec<Stmt>, output: &PathBuf) {
    let config = config::standard();

    let program = BinAstFile(code);

    let encoded: Vec<u8> = bincode::encode_to_vec(program, config).expect("Error serializando AST");

    std::fs::write(output, encoded).expect("Error escribiendo binario");
}

pub fn read_ast_from_bincode(input: &PathBuf) -> Vec<Stmt> {
    let bytes = std::fs::read(input).expect("No se pudo leer el archivo .mpc");
    let config = bincode::config::standard();

    let (program, _): (BinAstFile, usize) =
        bincode::decode_from_slice(&bytes, config).expect("Error decodificando binario");

    program.0
}

pub fn save_pairs_as_text(pairs: &Pairs<Rule>, output: &PathBuf) {
    std::fs::write(output, format!("{:#?}", pairs)).expect("No se pudo escribir archivo .mpp");
}

pub fn save_ast_as_text(code: Vec<Stmt>, output: &PathBuf) {
    std::fs::write(output, format!("{:#?}", code)).expect("No se pudo escribir archivo .mpa");
}
