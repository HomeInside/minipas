use super::ast::{Function, Stmt};
use super::declarations::parse_var_section;
use super::params::parse_param_list;
use super::program::parse_block;
use super::symbol_table::SymbolTable;
use super::types::parse_type;
use crate::Rule;
use pest::iterators::Pair;

// Parsea la declaración de función completa
pub fn parse_func_decl(pair: Pair<Rule>, sym_table: &mut SymbolTable) -> Function {
    //println!("parse_func_decl: entro");
    assert_eq!(pair.as_rule(), Rule::func_decl);
    let mut inner = pair.into_inner();

    // "function" keyword
    let _kw = inner.next().unwrap();
    // nombre de la función
    let name_pair = inner.next().unwrap();
    let name = name_pair.as_str().to_string();

    // "("
    let _lparen = inner.next().unwrap();

    // parámetros opcionales
    let next = inner.next().unwrap();
    let params = if next.as_rule() == Rule::param_list {
        let pl = parse_param_list(next);
        // ")" siguiente
        inner.next().unwrap();
        pl
    } else {
        // no hay parámetros, next debería ser ")"
        assert_eq!(next.as_rule(), Rule::rparen);
        Vec::new()
    };

    // ":" tipo de retorno
    let colon_pair = inner.next().unwrap();
    assert_eq!(colon_pair.as_rule(), Rule::colon);
    let type_pair = inner.next().unwrap();
    let ret_type = parse_type(type_pair);

    // ";" opcional antes del bloque
    let semicolon_pair = inner.next().unwrap();
    assert_eq!(semicolon_pair.as_rule(), Rule::semicolon);

    // var_section opcional
    let mut locals = Vec::new();
    let mut next_pair = inner.next().unwrap();
    if next_pair.as_rule() == Rule::var_section {
        locals = parse_var_section(Some(next_pair), sym_table);
        // el siguiente es el block
        next_pair = inner.next().unwrap();
    }

    // parse_block devuelve Stmt::Block, extraemos su contenido
    let Stmt::Block(body) = parse_block(next_pair, sym_table) else {
        panic!("El cuerpo de la función debe ser un bloque (begin...end)");
    };

    //println!("parse_func_decl: saliendo");
    Function {
        name,
        params,
        locals,
        return_type: ret_type,
        body,
    }
}
