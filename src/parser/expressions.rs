use super::ast::{Expr, Op};
use super::program::{check_ident, validate_identifier};
use super::symbol_table::SymbolTable;
use crate::Rule;
use pest::iterators::Pair;

pub fn parse_expr(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
    match pair.as_rule() {
        Rule::number => Expr::Number(pair.as_str().parse().unwrap()),
        Rule::ident => {
            let name = pair.as_str().to_string();
            validate_identifier(&name);
            //println!("parse_expr Rule::ident entro");
            check_ident(&name, sym_table);
            Expr::Ident(name)
        }
        Rule::expr | Rule::sum | Rule::product => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            let mut left = parse_expr(first, sym_table);

            while let Some(op_pair) = inner.next() {
                let right = parse_expr(inner.next().unwrap(), sym_table);
                let op = match op_pair.as_rule() {
                    Rule::add_op => Op::Add,
                    Rule::sub_op => Op::Sub,
                    Rule::mul_op => Op::Mul,
                    Rule::div_op => Op::Div,
                    _ => panic!("Operador inesperado en expr: {:?}", op_pair.as_rule()),
                };
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            left
        }
        Rule::factor => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::func_call => {
                    let mut ic = inner.into_inner();
                    let name_pair = ic.next().unwrap(); // ident
                    let name = name_pair.as_str().to_string();

                    let mut args: Vec<Expr> = Vec::new();
                    validate_identifier(&name);

                    for node in ic {
                        match node.as_rule() {
                            Rule::expr_list => {
                                for item in node.into_inner() {
                                    if item.as_rule() == Rule::expr_item {
                                        args.push(parse_expr_item(item, sym_table));
                                    }
                                }
                            }
                            Rule::comma => {} // ignorar comas
                            _ => {}           // ignorar otros nodos (lparen, rparen)
                        }
                    }

                    Expr::Call { name, args }
                }

                Rule::ident => {
                    let name = inner.as_str().to_string();
                    validate_identifier(&name);
                    //println!("parse_expr Rule::factor -> Rule::ident entro");
                    //check_ident(&name, sym_table);
                    Expr::Ident(name)
                }
                Rule::string_literal => {
                    let s = inner.as_str();
                    Expr::StringLiteral(s[1..s.len() - 1].to_string())
                }
                Rule::boolean_literal => {
                    let val = inner.as_str().to_lowercase() == "true";
                    Expr::BooleanLiteral(val)
                }
                Rule::number => Expr::Number(inner.as_str().parse().unwrap()),
                Rule::expr => parse_expr(inner, sym_table),

                _ => panic!("Factor inesperado: {:?}", inner.as_rule()),
            }
        }

        _ => panic!("parse_expr: Regla de expr no implementada: {:?}", pair.as_rule()),
    }
}

pub fn parse_expr_item(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
    assert_eq!(pair.as_rule(), Rule::expr_item);

    let inner = pair.into_inner().next().expect("expr_item vacío");

    match inner.as_rule() {
        Rule::expr => parse_expr(inner, sym_table),
        Rule::string_literal => {
            let s = inner.as_str();
            let s = &s[1..s.len() - 1]; // quitar comillas
            Expr::StringLiteral(s.to_string())
        }
        other => panic!("parse_expr_item Nodo inesperado dentro de expr_item: {:?}", other),
    }
}
