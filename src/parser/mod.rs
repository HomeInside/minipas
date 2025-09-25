pub mod ast;
pub mod keywords;
//#[allow(clippy::module_inception)]
//pub mod parser;

pub mod assignment;
pub mod conditionals;
pub mod declarations;
pub mod expressions;
pub mod functions;
pub mod params;
pub mod procedures;
pub mod program;
pub mod statements;
pub mod symbol_table;
pub mod types;
use pest::Parser;
use pest::iterators::Pairs;

use crate::MiniPasParser;
use crate::Rule;
use crate::parser::ast::Stmt;
use crate::parser::program::parse_program;
use crate::parser::symbol_table::SymbolTable;

pub fn gen_pairs(source: &str) -> Pairs<'_, Rule> {
    MiniPasParser::parse(Rule::program, source).unwrap_or_else(|e| panic!("Error de parseo: {} {}", source, e))
}

/// Genera el AST completo a partir del código fuente (para `run`, `check`, `build`)
pub fn gen_ast(source: &str) -> (Vec<Stmt>, SymbolTable) {
    let pairs = gen_pairs(source);
    //let (program, _) = parse_program(pairs);
    parse_program(pairs)
}
