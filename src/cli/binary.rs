use crate::parser::ast::Stmt;
use bincode::{Decode, Encode, config};
use std::path::PathBuf;

#[derive(Encode, Decode, Debug)]
pub struct BinAstFile(Vec<Stmt>);

pub fn gen_bincode_from_ast(code: Vec<Stmt>, output: &PathBuf) {
    let config = config::standard();

    let program = BinAstFile(code);
    //println!("program: {:#?}", program);

    let encoded: Vec<u8> = bincode::encode_to_vec(program, config).expect("Error serializando AST");
    //println!("encoded: {:?}", encoded);
    //println!("output: {:?}", output);

    std::fs::write(output, encoded).expect("Error escribiendo binario");
    println!("Generado binario {:?}", output);
}
