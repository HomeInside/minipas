use super::ast::Stmt;
use super::statements::parse_stmt;
use super::symbol_table::SymbolTable;
use super::types::parse_bool_expr;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_stmt_if(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    let mut inner = pair.into_inner();
    let mut cond = None;
    let mut then_branch = None;
    let mut else_branch = None;

    // TO FIX
    #[allow(clippy::while_let_on_iterator)]
    while let Some(p) = inner.next() {
        match p.as_rule() {
            Rule::bool_expr => cond = Some(parse_bool_expr(p, sym_table)),
            Rule::stmt | /*Rule::writeln_stmt |*/ Rule::assignment | Rule::var_decl | Rule::if_stmt => {
                if then_branch.is_none() {
                    then_branch = Some(parse_stmt(p, sym_table));
                } else {
                    else_branch = Some(parse_stmt(p, sym_table));
                }
            }
            Rule::keyword_if | Rule::keyword_then | Rule::keyword_else => {}
            other => panic!("Nodo inesperado en if_stmt: {:?}", other),
        }
    }

    Stmt::IfElse {
        cond: cond.expect("if sin condición"),
        then_branch: Box::new(then_branch.expect("if sin rama then")),
        else_branch: else_branch.map(Box::new),
    }
}
