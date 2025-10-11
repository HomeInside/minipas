use super::expressions::parse_expr;
use super::functions::parse_func_decl;
use super::keywords::KEYWORDS;
use super::procedures::parse_proc_decl;
use super::statements::parse_stmt;
use super::symbol_table::SymbolTable;
use crate::Rule;
use crate::parser::ast::{ForDir, Stmt, VarType, WhileStmt};
use crate::runtime::std_lib::builtins::BUILTINS;
use pest::iterators::{Pair, Pairs};

pub fn validate_identifier(name: &str) {
    if KEYWORDS.contains(&name) {
        panic!("Identificador '{}' no permitido: es una palabra reservada", name);
    }
}

//solo debería validar variables globales y builtins.
pub fn parse_block(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    assert_eq!(pair.as_rule(), Rule::block);
    let mut stmts = Vec::new();

    for p in pair.into_inner() {
        stmts.push(parse_stmt(p, sym_table));
    }

    Stmt::Block(stmts)
    //let body_stmts: Vec<Stmt> = parse_block(Stmt::Block(stmts))?; // devuelve Vec<Stmt>
}

pub fn check_ident(name: &str, sym_table: &SymbolTable) {
    if !sym_table.exists(name) && !BUILTINS.contains_key(name) {
        panic!("check_ident Variable o constante '{}' no declarada", name);
    }
}

pub fn parse_return_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    //println!("parse_return_stmt entro");
    assert_eq!(pair.as_rule(), Rule::return_stmt);
    // return ~ expr ~ semicolon -> el único hijo será expr
    let expr_pair = pair.into_inner().next().expect("return sin expresión");
    let expr = parse_expr(expr_pair, sym_table);
    Stmt::Return(expr)
}

pub fn parse_program(mut pairs: Pairs<Rule>) -> (Vec<Stmt>, SymbolTable) {
    //println!("parse_program entro");
    let program_pair = pairs.next().expect("No se encontró program");
    let mut sym_table = SymbolTable::with_builtins();

    let mut stmts: Vec<Stmt> = Vec::new();
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
                            Rule::keyword_byte => var_type = Some(VarType::Byte),
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
                //println!("parse_program entro al match");
                //println!("Rule::proc_decl entro");
                //println!("Rule::proc_decl p:{}", p.clone());

                let proc = parse_proc_decl(p, &mut sym_table);
                stmts.push(Stmt::ProcDecl {
                    name: proc.name.clone(),
                    params: proc.params.iter().map(|p| (p.name.clone(), p.ty.clone())).collect(), // Vec<(String, VarType)>
                    locals: proc.locals.iter().map(|p| (p.name.clone(), p.ty.clone())).collect(), // Vec<(String, VarType)>
                    body: proc.body.clone(),
                })
            }

            Rule::func_decl => {
                //println!("============");
                //println!("parse_program entro al match brazo Rule::func_decl entro");
                //println!("Rule::func_decl p:{}", p.clone());
                let func = parse_func_decl(p, &mut sym_table);
                stmts.push(Stmt::FuncDecl {
                    name: func.name.clone(),
                    params: func.params.iter().map(|p| (p.name.clone(), p.ty.clone())).collect(), // Vec<(String, VarType)>
                    locals: func.locals.iter().map(|p| (p.name.clone(), p.ty.clone())).collect(), // Vec<(String, VarType)>
                    return_type: func.return_type.clone(),                                        // 🟢 agregado
                    body: func.body.clone(),
                });
            }
            // for loop
            Rule::for_stmt => {
                //println!("parse_program: brazo Rule::for_stmt entro");
                let mut inner = pairs.clone();

                let var = inner.next().unwrap().as_str().to_string(); // ident
                inner.next(); // := (assign_op)

                let start = parse_expr(inner.next().unwrap(), &sym_table);
                let dir_token = inner.next().unwrap(); // "to" o "downto"

                let direction = if dir_token.as_rule() == Rule::keyword_to {
                    ForDir::To
                } else {
                    ForDir::DownTo
                };

                let end = parse_expr(inner.next().unwrap(), &sym_table);
                inner.next(); // "do"
                let body = parse_stmt(inner.next().unwrap(), &sym_table);

                stmts.push(Stmt::For {
                    var,
                    start,
                    end,
                    direction,
                    body: Box::new(body),
                })
            }
            // while loop
            Rule::while_stmt => {
                //println!("parse_program: brazo Rule::while_stmt entro");
                let mut inner = p.into_inner();

                let cond_pair = inner.next().expect("Se esperaba condición en while");
                let condition = parse_expr(cond_pair, &sym_table);

                let body_pair = inner.next().expect("Se esperaba cuerpo del while");
                let body = parse_stmt(body_pair, &sym_table);

                stmts.push(Stmt::While(WhileStmt {
                    condition,
                    body: Box::new(body),
                }));
            }
            // repeat loop
            Rule::repeat_stmt => {
                //println!("============");
                //println!("parse_program entro al match brazo Rule::repeat_stmt entro");
                let stmt = parse_stmt(p, &sym_table);
                //println!("parse_program stmt {:?}", stmt);
                stmts.push(stmt);
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
