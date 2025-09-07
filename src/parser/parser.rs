use crate::Rule;
use crate::{Expr, Op, Stmt};
use pest::iterators::{Pair, Pairs};

pub fn parse_program(mut pairs: Pairs<Rule>) -> Vec<Stmt> {
    let program_pair = pairs.next().expect("No se encontró program");

    // buscar el primer block en los hijos del programa
    let block_pair = program_pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::block)
        .expect("No se encontró el bloque principal");

    if let Stmt::Block(stmts) = parse_block(block_pair) {
        stmts
    } else {
        panic!("El bloque principal no retornó un Block");
    }
}

fn parse_block(pair: Pair<Rule>) -> Stmt {
    assert_eq!(pair.as_rule(), Rule::block);
    let mut stmts = Vec::new();

    for p in pair.into_inner() {
        match p.as_rule() {
            //Rule::dot => {}                 // ignorar el punto final
            _ => stmts.push(parse_stmt(p)), // delegar a parse_stmt
        }
    }

    Stmt::Block(stmts)
}
pub fn parse_stmt(pair: Pair<Rule>) -> Stmt {
    match pair.as_rule() {
        Rule::stmt => {
            let inner = pair.into_inner().next().expect("stmt vacío");
            parse_stmt(inner)
        }
        Rule::var_decl => {
            let vars = pair
                .into_inner()
                .filter(|p| p.as_rule() == Rule::ident)
                .map(|p| p.as_str().to_string())
                .collect();
            Stmt::VarDecl(vars)
        }
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().expect("assignment sin ident").as_str().to_string();
            inner.next(); // saltar assign_op
            let expr_pair = inner.next().expect("assignment sin expr");
            let expr = parse_expr(expr_pair);
            Stmt::Assign(name, expr)
        }
        Rule::writeln_stmt => {
            let mut exprs = Vec::new();

            for p in pair.into_inner() {
                match p.as_rule() {
                    Rule::expr_list => {
                        for item in p.into_inner() {
                            match item.as_rule() {
                                Rule::expr_item => {
                                    let mut inner = item.into_inner();
                                    let inner_item = inner.next().unwrap();
                                    match inner_item.as_rule() {
                                        Rule::expr => exprs.push(parse_expr(inner_item)),
                                        Rule::string => exprs.push(Expr::StringLiteral(
                                            inner_item.as_str().trim_matches('"').to_string(),
                                        )),
                                        other => panic!("Nodo inesperado dentro de expr_item: {:?}", other),
                                    }
                                }
                                Rule::comma => {}
                                other => panic!("Nodo inesperado en expr_list: {:?}", other),
                            }
                        }
                    }
                    Rule::keyword_writeln | Rule::lparen | Rule::rparen | Rule::semicolon => {
                        // ignorar
                    }
                    other => panic!("Nodo inesperado en writeln_stmt: {:?}", other),
                }
            }

            //Stmt::Writeln(exprs)
            Stmt::WritelnList(exprs)
        }

        Rule::if_stmt => {
            let mut inner = pair.into_inner();
            let mut cond = None;
            let mut then_branch = None;
            let mut else_branch = None;

            while let Some(p) = inner.next() {
                match p.as_rule() {
                    Rule::bool_expr => cond = Some(parse_bool_expr(p)),
                    Rule::stmt | Rule::writeln_stmt | Rule::assignment | Rule::var_decl | Rule::if_stmt => {
                        if then_branch.is_none() {
                            then_branch = Some(parse_stmt(p));
                        } else {
                            else_branch = Some(parse_stmt(p));
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
        Rule::EOI => {
            // Ignorar fin/inicio de input
            // No devolvemos un Stmt válido aquí
            panic!("parse_stmt no debería recibir SOI/EOI directamente");
        }

        other => panic!("Regla inesperada en stmt: {:?}", other),
    }
}

pub fn parse_bool_expr(pair: Pair<Rule>) -> Expr {
    assert_eq!(pair.as_rule(), Rule::bool_expr);
    let mut inner = pair.into_inner();
    let left_pair = inner.next().expect("bool_expr sin lado izquierdo");
    let left = match left_pair.as_rule() {
        Rule::expr => parse_expr(left_pair),
        Rule::number => Expr::Number(left_pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Ident(left_pair.as_str().to_string()),
        other => panic!("left inesperado en bool_expr: {:?}", other),
    };

    let op_pair = inner.next().expect("bool_expr sin operador");
    let right_pair = inner.next().expect("bool_expr sin lado derecho");
    let right = match right_pair.as_rule() {
        Rule::expr => parse_expr(right_pair),
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

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr | Rule::sum | Rule::product => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();
            let mut left = parse_expr(first);

            while let Some(op_pair) = inner.next() {
                let right = parse_expr(inner.next().unwrap());
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
                Rule::number => Expr::Number(inner.as_str().parse().unwrap()),
                Rule::ident => Expr::Ident(inner.as_str().to_string()),
                Rule::expr => parse_expr(inner),
                _ => panic!("Factor inesperado en expr: {:?}", inner.as_rule()),
            }
        }
        Rule::number => Expr::Number(pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Ident(pair.as_str().to_string()),
        _ => panic!("Nodo inesperado en parse_expr: {:?}", pair.as_rule()),
    }
}
