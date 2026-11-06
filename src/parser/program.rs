use super::expressions::parse_expr;
use super::functions::parse_func_decl;
use super::keywords::KEYWORDS;
use super::procedures::parse_proc_decl;
use super::statements::parse_stmt;
use super::symbol_table::SymbolTable;
use crate::Rule;
use crate::parser::ast::Value;
use crate::parser::ast::{Stmt, VarType};
use crate::runtime::std_lib::builtins::{BUILTINS, Builtin};
use pest::iterators::{Pair, Pairs};

/// valida que no sea una palabra reservada
pub fn validate_identifier(name: &str) {
    assert!(
        !KEYWORDS.contains(&name),
        "Identificador '{}' no permitido: es una palabra reservada",
        name
    );
}

/// valida constantes declaradas con `const`
pub fn validate_const_name(name: &str) {
    let ok = name
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    assert!(
        ok,
        "Nombre de constante inválido '{}': debe usar solo MAYÚSCULAS, dígitos y '_'",
        name
    );
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
    assert!(
        !(!sym_table.exists(name) && !BUILTINS.contains_key(name) && !sym_table.is_constant(name)),
        "check_ident Variable o constante '{}' no declarada",
        name
    );
}

pub fn parse_return_stmt(pair: Pair<Rule>, sym_table: &SymbolTable) -> Stmt {
    //println!("parse_return_stmt entro");
    assert_eq!(pair.as_rule(), Rule::return_stmt);
    // return ~ expr ~ semicolon -> el único hijo será expr
    let expr_pair = pair.into_inner().next().expect("return sin expresión");
    let expr = parse_expr(expr_pair, sym_table);
    Stmt::Return(expr)
}

/// valida el tipo de dato de las constantes definidas por el usuario
fn parse_const_literal(pair: Pair<Rule>) -> (VarType, Value) {
    let inner = pair.into_inner().next().expect("const_literal vacío");
    match inner.as_rule() {
        Rule::number => {
            let lit = inner.as_str();
            if lit.contains('.') {
                (VarType::Real, Value::Real(lit.parse::<f64>().unwrap()))
            } else {
                (VarType::Integer, Value::Integer(lit.parse::<i64>().unwrap()))
            }
        }
        Rule::string_literal => {
            let s = inner.as_str();
            (VarType::Str, Value::Str(s[1..s.len() - 1].to_string()))
        }
        Rule::boolean_literal => {
            let b = inner.as_str().to_lowercase() == "true";
            (VarType::Boolean, Value::Boolean(b))
        }
        other => panic!("const_literal inesperado: {:?}", other),
    }
}

/// Genera el AST y la tabla de simbolos del programa.
///
/// # Arguments
///
/// * `pairs: Pairs<Rule>` - los pairs(reglas) generados
///   por `MiniPasParser::parse`
///
/// # Returns
///
/// una tupla `(Vec<Stmt>, SymbolTable)`, la lista de
///   sentencias(AST) y la tabla de simbolos.
///
/// # Examples
///
/// ```rust,no_run,ignore
/// use crate::parser::program::parse_program;
/// ...
///
/// parse_program(pairs);
/// ```
///
pub fn parse_program(mut pairs: Pairs<Rule>) -> (Vec<Stmt>, SymbolTable) {
    //println!("parse_program entro");
    let program_pair = pairs.next().expect("No se encontró program");
    let mut sym_table = SymbolTable::with_builtins();

    let mut stmts: Vec<Stmt> = Vec::new();
    let mut block_pair_opt = None;

    for p in program_pair.into_inner() {
        match p.as_rule() {
            Rule::const_section => {
                for decl in p.into_inner() {
                    /*
                    lista de declaraciones
                    for q in decl.clone().into_inner() {
                        eprintln!("const_decl child: {:?} = {:?}", q.as_rule(), q.as_str());
                    }
                    println!();
                    */
                    //en caso de que no sea una constante
                    if decl.as_rule() != Rule::const_decl {
                        continue;
                    }

                    let mut inner = decl.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();

                    // no permite redefinir las builtins
                    if let Some(b) = BUILTINS.get(&name) {
                        let kind = match b {
                            Builtin::Const(_) => "constant",
                            Builtin::Func(_) => "function",
                            Builtin::Proc(_) => "procedure",
                        };
                        panic!(
                            "No se puede declarar '{}': ya existe como 'builtin {}'(uso interno del lenguaje) y no puede redefinirse.\n\
                             Sugerencia: use otro nombre (p. ej. '{}_USER') o elimine esta declaración.",
                            name, kind, name
                        );
                    }

                    // el siguiente token es "=", saltamos
                    let lit_pair = inner.next().unwrap();
                    let (ty, val) = parse_const_literal(lit_pair);

                    validate_identifier(&name);
                    // validate que las constantes esten en mayusculas
                    validate_const_name(&name);

                    // agrega la constante a la tabla de simbolos
                    sym_table
                        .declare_const(&name, ty, val)
                        .unwrap_or_else(|e| panic!("{}", e));
                }
            }
            //
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
                });
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
