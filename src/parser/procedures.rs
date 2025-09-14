use super::ast::Procedure;
use super::ast::Stmt;
use super::program::parse_block;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

// procedures y functions
pub fn parse_proc_decl(pair: Pair<Rule>, sym_table: &SymbolTable) -> Procedure {
    let mut inner = pair.into_inner();

    // el primero siempre es keyword_procedure → lo ignoramos
    let _kw = inner.next().unwrap();

    // ahora sí, el nombre
    let name = inner.next().unwrap().as_str().to_string();
    let mut params = Vec::new();
    let mut body = Vec::new();

    for p in inner {
        match p.as_rule() {
            Rule::param_list => {
                for id in p.into_inner() {
                    if id.as_rule() == Rule::ident {
                        params.push(id.as_str().to_string());
                    }
                }
            }
            Rule::block => {
                body = match parse_block(p, sym_table) {
                    Stmt::Block(stmts) => stmts,
                    _ => panic!("bloque inválido en procedure"),
                };
            }
            _ => {}
        }
    }

    Procedure { name, params, body }
}
