use super::ast::Stmt;
use super::expressions::parse_expr;
use super::program::validate_identifier;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_assignment(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    let mut inner = pair.into_inner(); // ident ~ ":=" ~ expr ~ ";"
    let ident = inner.next().unwrap().as_str().to_string();

    validate_identifier(&ident);

    inner.next(); // skip ":="

    let expr_pair = inner.next().unwrap();
    let expr = parse_expr(expr_pair, sym_table);
    Stmt::Assign(ident, expr)
}
