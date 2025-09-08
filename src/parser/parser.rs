use crate::Rule;
use crate::{Expr, Op, Stmt, VarType};
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub struct SymbolTable {
    variables: HashMap<String, VarType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    /*pub fn declare(&mut self, name: &str, typ: &str) -> Result<(), String> {
        if self.variables.contains_key(name) {
            return Err(format!("Variable '{}' ya declarada", name));
        }
        self.variables.insert(name.to_string(), typ.to_string());
        Ok(())
    }*/
    pub fn declare(&mut self, name: &str, typ: VarType) -> Result<(), String> {
        if self.variables.contains_key(name) {
            return Err(format!("Variable '{}' ya declarada", name));
        }
        self.variables.insert(name.to_string(), typ);
        Ok(())
    }

    pub fn get_type(&self, name: &str) -> Option<&VarType> {
        self.variables.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

pub fn parse_program(mut pairs: Pairs<Rule>) -> (Vec<Stmt>, SymbolTable) {
    let program_pair = pairs.next().expect("No se encontró program");
    let mut sym_table = SymbolTable::new();

    let mut block_pair_opt = None;

    for p in program_pair.into_inner() {
        match p.as_rule() {
            Rule::var_section => {
                // Parsear todas las declaraciones y llenar sym_table
                for decl in p.into_inner() {
                    // cada decl es un var_decl
                    let mut idents = Vec::new();
                    let mut var_type = None;

                    for part in decl.into_inner() {
                        match part.as_rule() {
                            Rule::ident => idents.push(part.as_str().to_string()),
                            Rule::keyword_integer => var_type = Some(VarType::Integer),
                            Rule::keyword_real => var_type = Some(VarType::Real),
                            Rule::keyword_string => var_type = Some(VarType::Str),
                            Rule::keyword_boolean => var_type = Some(VarType::Boolean),
                            _ => {}
                        }
                    }

                    let var_type = var_type.expect("Declaración sin tipo");

                    for name in idents {
                        sym_table
                            .declare(&name, var_type.clone())
                            .expect("Error al declarar variable");
                    }
                }
            }

            Rule::block => block_pair_opt = Some(p),
            _ => {}
        }
    }

    let block_pair = block_pair_opt.expect("No se encontró el bloque principal");

    if let Stmt::Block(stmts) = parse_block(block_pair, &sym_table) {
        (stmts, sym_table)
    } else {
        panic!("El bloque principal no retornó un Block");
    }
}

fn parse_block(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    assert_eq!(pair.as_rule(), Rule::block);
    let mut stmts = Vec::new();

    for p in pair.into_inner() {
        stmts.push(parse_stmt(p, sym_table));
    }

    Stmt::Block(stmts)
}

pub fn parse_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let name = inner.next().expect("assignment sin ident").as_str().to_string();
            if !sym_table.exists(&name) {
                panic!("Variable '{}' no declarada", name);
            }
            inner.next(); // saltar assign_op
            let expr_pair = inner.next().expect("assignment sin expr");
            let expr = parse_expr(expr_pair, sym_table);
            Stmt::Assign(name, expr)
        }
        Rule::var_decl => {
            panic!("Declaración de variables no permitida dentro de begin…end");
        }
        Rule::block => parse_block(pair, sym_table),
        Rule::stmt => {
            let inner = pair.into_inner().next().expect("stmt vacío");
            parse_stmt(inner, sym_table)
        }
        Rule::writeln_stmt => parse_stmt_writeln(pair, sym_table),
        Rule::if_stmt => parse_stmt_if(pair, sym_table),
        Rule::EOI => {
            // Ignorar fin/inicio de input
            // No devolvemos un Stmt válido aquí
            panic!("parse_stmt no debería recibir SOI/EOI directamente");
        }
        other => panic!("Regla inesperada en stmt: {:?}", other),
        //_ => unimplemented!(),
    }
}

pub fn parse_bool_expr(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
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

fn parse_expr(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
    match pair.as_rule() {
        Rule::number => Expr::Number(pair.as_str().parse().unwrap()),
        Rule::ident => {
            let name = pair.as_str().to_string();
            if !sym_table.exists(&name) {
                panic!("Variable '{}' no declarada", name);
            }
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
                Rule::ident => {
                    let name = inner.as_str().to_string();
                    /*if !sym_table.exists(&name) {
                        panic!("Variable '{}' no declarada", name);
                    }*/
                    // 🔥 Hack rápido: aceptar PI (y otros)
                    let allowed_consts = ["PI", "TRUE", "FALSE"];
                    if !sym_table.exists(&name) && !allowed_consts.contains(&name.as_str()) {
                        panic!("Variable '{}' no declarada", name);
                    }

                    /*if !sym_table.exists(&name) && name != "PI" {
                        // ⚠️ comentar o quitar el panic mientras tanto
                        panic!("Variable '{}' no declarada", name);
                    }*/
                    Expr::Ident(name)
                }
                Rule::string_literal => {
                    let s = inner.as_str();
                    Expr::StringLiteral(s[1..s.len() - 1].to_string()) // quitar comillas
                }
                Rule::boolean_literal => {
                    let val = inner.as_str().to_lowercase() == "true";
                    Expr::BooleanLiteral(val)
                }

                //Rule::number => Expr::Number(inner.as_str().parse().unwrap()),
                Rule::number => {
                    let n: f64 = inner.as_str().parse().unwrap();
                    Expr::Number(n)
                }

                Rule::expr => parse_expr(inner, sym_table),
                _ => panic!("Factor inesperado: {:?}", inner.as_rule()),
            }
        }
        _ => panic!("Regla de expr no implementada: {:?}", pair.as_rule()),
    }
}

pub fn parse_stmt_if(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    let mut inner = pair.into_inner();
    let mut cond = None;
    let mut then_branch = None;
    let mut else_branch = None;

    while let Some(p) = inner.next() {
        match p.as_rule() {
            Rule::bool_expr => cond = Some(parse_bool_expr(p, sym_table)),
            Rule::stmt | Rule::writeln_stmt | Rule::assignment | Rule::var_decl | Rule::if_stmt => {
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

pub fn parse_stmt_writeln(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
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
                                Rule::expr => exprs.push(parse_expr(inner_item, sym_table)),
                                Rule::string => {
                                    exprs.push(Expr::StringLiteral(inner_item.as_str().trim_matches('"').to_string()))
                                }
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
