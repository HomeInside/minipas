use super::assignment::parse_assignment;
use super::ast::Stmt;
use super::conditionals::parse_stmt_if;
use super::expressions::parse_expr;
use super::program::parse_block;
use super::program::parse_return_stmt;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    match pair.as_rule() {
        Rule::var_decl => {
            panic!("Declaración de variables no permitida dentro de begin…end");
        }
        Rule::stmt => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::assignment => parse_assignment(inner, sym_table),
                Rule::if_stmt => parse_stmt_if(inner, sym_table),
                Rule::block => parse_block(inner, sym_table), //parse_block(inner),
                // permite statements de expresiones (como writeln(...);)
                Rule::expr_stmt => {
                    // 👈 NUEVO
                    let expr_pair = inner.into_inner().next().unwrap();
                    let expr = parse_expr(expr_pair, sym_table);
                    Stmt::Expr(expr)
                }
                Rule::return_stmt => parse_return_stmt(inner, sym_table),

                other => panic!("Regla inesperada en stmt: {:?}", other),
            }
        }
        Rule::EOI => {
            // Ignorar fin/inicio de input
            // no hay un Stmt válido
            panic!("parse_stmt no debería recibir SOI/EOI directamente");
        }
        other => panic!("Regla inesperada en stmt: {:?}", other),
    }
}
