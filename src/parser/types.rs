use super::ast::{Expr, Op, VarType};
use super::expressions::parse_expr;
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

// Parsea un tipo: integer, real, boolean, string
pub fn parse_type(pair: Pair<Rule>) -> VarType {
    //println!("parse_type: entro");
    match pair.as_rule() {
        Rule::type_keyword => {
            // tomar el hijo que es realmente keyword_integer, etc.
            let inner = pair.into_inner().next().unwrap();
            parse_type(inner)
        }
        Rule::keyword_integer => VarType::Integer,
        Rule::keyword_real => VarType::Real,
        Rule::keyword_boolean => VarType::Boolean,
        Rule::keyword_string => VarType::Str,
        Rule::keyword_nil => VarType::Nil,
        _ => panic!("parse_type: tipo desconocido `{}`", pair.as_str()),
    }
}

pub fn parse_bool_expr(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
    //println!("parse_bool_expr: entro");
    assert_eq!(pair.as_rule(), Rule::bool_expr);
    let mut inner = pair.into_inner();
    let left_pair = inner.next().expect("bool_expr sin lado izquierdo");
    let left = match left_pair.as_rule() {
        Rule::expr => parse_expr(left_pair, sym_table),
        Rule::number => Expr::Number(left_pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Ident(left_pair.as_str().to_string()),
        other => panic!("left inesperado en bool_expr: {:?}", other),
    };

    let op_pair = inner.next().expect("bool_expr sin operador");
    let right_pair = inner.next().expect("bool_expr sin lado derecho");
    let right = match right_pair.as_rule() {
        Rule::expr => parse_expr(right_pair, sym_table),
        Rule::number => Expr::Number(right_pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Ident(right_pair.as_str().to_string()),
        other => panic!("right inesperado en bool_expr: {:?}", other),
    };

    let op = match op_pair.as_str() {
        ">" => Op::Greater,
        "<" => Op::Less,
        ">=" => Op::GreaterEq,
        "<=" => Op::LessEq,
        "=" => Op::Equal,
        "<>" => Op::NotEqual,
        _ => panic!("Operador de comparación no soportado: {}", op_pair.as_str()),
    };

    Expr::BinaryOp {
        left: Box::new(left),
        op,
        right: Box::new(right),
    }
}
