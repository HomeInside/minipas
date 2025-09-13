use super::keywords::KEYWORDS;
use crate::Rule;
use crate::parser::ast::{Function, Procedure};
use crate::runtime::std_lib::builtins::BUILTINS;
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
    pub fn declare(&mut self, name: &str, typ: VarType) -> Result<(), String> {
        if self.variables.contains_key(name) {
            return Err(format!("Variable '{}' ya declarada", name));
        }
        self.variables.insert(name.to_string(), typ);
        Ok(())
    }

    // TO FIX
    #[allow(dead_code)]
    pub fn get_type(&self, name: &str) -> Option<&VarType> {
        self.variables.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

fn validate_identifier(name: &str) {
    if KEYWORDS.contains(&name) {
        panic!("Identificador '{}' no permitido: es una palabra reservada", name);
    }
}

pub fn parse_program(mut pairs: Pairs<Rule>) -> (Vec<Stmt>, SymbolTable) {
    let program_pair = pairs.next().expect("No se encontró program");
    let mut sym_table = SymbolTable::new();
    let mut stmts: Vec<Stmt> = Vec::new(); // 👈 AST completo del programa
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
                        validate_identifier(&name);
                        sym_table
                            .declare(&name, var_type.clone())
                            .expect("Error al declarar variable");
                    }
                }
            }

            Rule::block => block_pair_opt = Some(p),
            Rule::proc_decl => {
                //println!("============");
                //println!("Rule::proc_decl entro");
                //println!("Rule::proc_decl p:{}", p.clone());
                let proc = parse_proc_decl(p, &sym_table);

                stmts.push(Stmt::ProcDecl {
                    name: proc.name,
                    params: proc.params,
                    body: proc.body,
                })
                //env.procs.insert(proc.name.clone(), proc);
            }

            Rule::func_decl => {
                // 👈 NUEVO
                println!("============");
                println!("parse_program entro al match");
                println!("Rule::func_decl entro");
                println!("Rule::func_decl p:{}", p.clone());
                let func = parse_func_decl(p, &sym_table);
                stmts.push(Stmt::FuncDecl {
                    name: func.name,
                    params: func.params,
                    return_type: func.return_type,
                    body: func.body,
                });
            }

            _ => {}
        }
    }

    let block_pair = block_pair_opt.expect("No se encontró el bloque principal");

    if let Stmt::Block(block_stmts) = parse_block(block_pair, &sym_table) {
        stmts.push(Stmt::Block(block_stmts)); // 👈 agregamos el bloque principal al final
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

fn parse_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
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

fn parse_bool_expr(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
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
            validate_identifier(&name);
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
                    check_ident(&name, sym_table);
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

fn parse_stmt_if(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
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

fn parse_expr_item(pair: Pair<Rule>, sym_table: &SymbolTable) -> Expr {
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

fn parse_assignment(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    let mut inner = pair.into_inner(); // ident ~ ":=" ~ expr ~ ";"
    let ident = inner.next().unwrap().as_str().to_string();

    validate_identifier(&ident);

    inner.next(); // skip ":="

    let expr_pair = inner.next().unwrap();
    let expr = parse_expr(expr_pair, sym_table);
    Stmt::Assign(ident, expr)
}

fn check_ident(name: &str, sym_table: &SymbolTable) {
    if !sym_table.exists(name) && !BUILTINS.contains_key(name) {
        panic!("Variable o constante '{}' no declarada", name);
    }
}

// procedures y functions
fn parse_proc_decl(pair: Pair<Rule>, sym_table: &SymbolTable) -> Procedure {
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

// 👈 NUEVO
fn parse_func_decl(pair: Pair<Rule>, sym_table: &SymbolTable) -> Function {
    println!("parse_func_decl entro");
    assert_eq!(pair.as_rule(), Rule::func_decl);
    let mut inner = pair.into_inner();

    // salta el "function"
    inner.next();

    // ahora sí, el identificador
    let name_pair = inner.next().unwrap();
    let name = name_pair.as_str().to_string();

    let mut params = Vec::new();
    let mut body = Vec::new();
    let mut return_type = VarType::Nil; // default

    // … luego params, return_type, body, etc
    for p in inner {
        match p.as_rule() {
            Rule::param_list => {
                for id in p.into_inner() {
                    if id.as_rule() == Rule::ident {
                        params.push(id.as_str().to_string());
                    }
                }
            }
            Rule::keyword_integer => return_type = VarType::Integer,
            Rule::keyword_real => return_type = VarType::Real,
            Rule::keyword_string => return_type = VarType::Str,
            Rule::keyword_boolean => return_type = VarType::Boolean,
            Rule::keyword_nil => return_type = VarType::Nil,
            Rule::block => {
                body = match parse_block(p, sym_table) {
                    Stmt::Block(stmts) => stmts,
                    _ => panic!("parse_func_decl bloque inválido en function"),
                };
            }
            _ => {}
        }
    }
    //println!("==========parse_func_decl block return==========");
    //println!("parse_func_decl block return name: {}", name);
    //println!("parse_func_decl block return params: {:?}", params);
    //println!("parse_func_decl block return return_type: {:?}", return_type);
    //println!("parse_func_decl block return body: {:?}", body);
    if matches!(return_type, VarType::Nil) {
        panic!("Función '{}' sin tipo de retorno", name);
    }
    Function {
        name,
        params,
        return_type,
        body,
    }
}

// 👈 NUEVO
fn parse_return_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    println!("parse_return_stmt entro");
    assert_eq!(pair.as_rule(), Rule::return_stmt);
    // return ~ expr ~ semicolon -> el único hijo será expr
    let expr_pair = pair.into_inner().next().expect("return sin expresión");
    let expr = parse_expr(expr_pair, sym_table);
    Stmt::Return(expr)
}
