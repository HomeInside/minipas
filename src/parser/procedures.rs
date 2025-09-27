use super::ast::{Procedure, Stmt};
use super::declarations::parse_var_section;
use super::params::parse_param_list;
use super::program::parse_block;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

// procedures
pub fn parse_proc_decl(pair: Pair<Rule>, sym_table: &mut SymbolTable) -> Procedure {
    //println!("parse_proc_decl: entro");
    assert_eq!(pair.as_rule(), Rule::proc_decl);
    let mut inner = pair.into_inner();

    // "procedure" keyword_procedure
    let _kw = inner.next().unwrap();

    // nombre del procedure
    let name = inner.next().unwrap().as_str().to_string();

    // "("
    let _lparen = inner.next().unwrap();

    // parámetros opcionales
    let next = inner.next().unwrap();

    let params = if next.as_rule() == Rule::param_list {
        let pl = parse_param_list(next);

        for p in &pl {
            sym_table.insert(p.name.clone(), p.ty.clone());
        }

        // ")" siguiente
        inner.next().unwrap();
        pl
    } else {
        // no hay parámetros, next debería ser ")"
        assert_eq!(next.as_rule(), Rule::rparen);
        Vec::new()
    };

    // ";" opcional antes del bloque
    let semicolon_pair = inner.next().unwrap();
    assert_eq!(semicolon_pair.as_rule(), Rule::semicolon);

    // var_section opcional
    let mut locals = Vec::new();

    let mut next_pair = inner.next().unwrap();

    if next_pair.as_rule() == Rule::var_section {
        let pv = parse_var_section(Some(next_pair), sym_table);
        locals.extend(pv);
        // el siguiente es el block
        next_pair = inner.next().unwrap();
    }

    // body
    let Stmt::Block(body) = parse_block(next_pair, sym_table) else {
        panic!("El cuerpo de la función debe ser un bloque (begin...end)");
    };

    Procedure {
        name,
        params,
        locals,
        body,
    }
}
