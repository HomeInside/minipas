use super::ast::{Expr, Op, Stmt};
use super::expressions::parse_expr;
use super::program::validate_identifier;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_assignment(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();

    validate_identifier(&ident);

    let op_pair = inner.next().unwrap();
    let expr = parse_expr(inner.next().unwrap(), sym_table);
    inner.next(); // skip ";"
    // 👇 Nuevo
    match op_pair.as_rule() {
        Rule::assign_op => Stmt::Assign(ident, expr),
        Rule::assign_add => {
            let rhs = Expr::BinaryOp {
                left: Box::new(Expr::Ident(ident.clone())),
                op: Op::Add,
                right: Box::new(expr),
            };
            Stmt::Assign(ident, rhs)
        }
        Rule::assign_sub => {
            let rhs = Expr::BinaryOp {
                left: Box::new(Expr::Ident(ident.clone())),
                op: Op::Sub,
                right: Box::new(expr),
            };
            Stmt::Assign(ident, rhs)
        }
        Rule::assign_mul => {
            let rhs = Expr::BinaryOp {
                left: Box::new(Expr::Ident(ident.clone())),
                op: Op::Mul,
                right: Box::new(expr),
            };
            Stmt::Assign(ident, rhs)
        }
        Rule::assign_div => {
            let rhs = Expr::BinaryOp {
                left: Box::new(Expr::Ident(ident.clone())),
                op: Op::Div,
                right: Box::new(expr),
            };
            Stmt::Assign(ident, rhs)
        }
        Rule::assign_mod => {
            let rhs = Expr::BinaryOp {
                left: Box::new(Expr::Ident(ident.clone())),
                op: Op::Mod,
                right: Box::new(expr),
            };
            Stmt::Assign(ident, rhs)
        }
        _ => panic!("Operador de asignación desconocido"),
    }
}
