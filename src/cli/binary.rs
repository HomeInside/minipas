use crate::parser::ast::Stmt;
use crate::parser::symbol_table::SymbolTable;
use bincode::{Decode, Encode, config};
use std::path::PathBuf;

// pub struct BinAstFile(Vec<Stmt>);
#[derive(Encode, Decode, Debug)]
pub struct BinAstFile {
    code: Vec<Stmt>,
    symbols: SymbolTable,
}

/// Serializa y Guarda el AST y la tabla de simbolos
/// a un archivo en formato .mpc
///
pub fn gen_bincode_from_ast(code: Vec<Stmt>, symbols: SymbolTable, output: &PathBuf) {
    let config = config::standard();

    let program = BinAstFile { code, symbols };

    let encoded: Vec<u8> = bincode::encode_to_vec(program, config).expect("Error serializando AST");

    std::fs::write(output, encoded).expect("Error escribiendo binario");
}

/// Deserializa y obtiene de un archivo .mpc, el AST y
/// la tabla de simbolos
///
pub fn read_ast_from_bincode(input: &PathBuf) -> (Vec<Stmt>, SymbolTable) {
    let bytes = std::fs::read(input).expect("No se pudo leer el archivo .mpc");
    let config = bincode::config::standard();

    let (program, _): (BinAstFile, usize) =
        bincode::decode_from_slice(&bytes, config).expect("Error decodificando binario");

    (program.code, program.symbols)
}
