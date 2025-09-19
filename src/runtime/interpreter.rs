use crate::parser::ast::{Function, Param, Procedure, VarType};
use crate::runtime::std_lib::builtins::Builtin;
use crate::{Expr, Op, Stmt, Value};
use std::collections::HashMap;
pub type Environment = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    pub vars: Environment,
    pub funcs: HashMap<String, Function>, // 👈 nuevo
    pub procs: HashMap<String, Procedure>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Self {
            vars: Environment::new(),
            procs: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct ReturnError(pub Value); // wrapper para salir de la función

impl std::fmt::Display for ReturnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Return: {}", self.0)
    }
}

impl std::error::Error for ReturnError {}

fn apply_op(l: Value, op: &Op, r: Value) -> Value {
    match (l, r) {
        (Value::Integer(li), Value::Integer(ri)) => match op {
            Op::Add => Value::Integer(li + ri),
            Op::Sub => Value::Integer(li - ri),
            Op::Mul => Value::Integer(li * ri),
            Op::Div => Value::Real(li as f64 / ri as f64),
            Op::Greater => Value::Boolean(li > ri),
            Op::Less => Value::Boolean(li < ri),
            Op::GreaterEq => Value::Boolean(li >= ri),
            Op::LessEq => Value::Boolean(li <= ri),
            Op::Equal => Value::Boolean(li == ri),
            Op::NotEqual => Value::Boolean(li != ri),
        },

        (Value::Real(lf), Value::Real(rf)) => match op {
            Op::Add => Value::Real(lf + rf),
            Op::Sub => Value::Real(lf - rf),
            Op::Mul => Value::Real(lf * rf),
            Op::Div => Value::Real(lf / rf),
            Op::Greater => Value::Boolean(lf > rf),
            Op::Less => Value::Boolean(lf < rf),
            Op::GreaterEq => Value::Boolean(lf >= rf),
            Op::LessEq => Value::Boolean(lf <= rf),
            Op::Equal => Value::Boolean(lf == rf),
            Op::NotEqual => Value::Boolean(lf != rf),
        },

        (Value::Integer(li), Value::Real(rf)) => apply_op(Value::Real(li as f64), op, Value::Real(rf)),
        (Value::Real(lf), Value::Integer(ri)) => apply_op(Value::Real(lf), op, Value::Real(ri as f64)),

        (Value::Str(ls), Value::Str(rs)) => match op {
            Op::Add => Value::Str(ls + &rs),
            Op::Equal => Value::Boolean(ls == rs),
            Op::NotEqual => Value::Boolean(ls != rs),
            _ => panic!("Operador no soportado para strings"),
        },

        (Value::Boolean(lb), Value::Boolean(rb)) => match op {
            Op::Equal => Value::Boolean(lb == rb),
            Op::NotEqual => Value::Boolean(lb != rb),
            _ => panic!("Operador no soportado para booleanos"),
        },

        _ => panic!("Operación inválida entre tipos"),
    }
}

pub fn eval_expr(expr: &Expr, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) -> Value {
    //println!("eval_expr entro a la funcion");
    //println!("eval_expr expr: {:?}", expr);
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                Value::Integer(*n as i64)
            } else {
                Value::Real(*n)
            }
        }
        Expr::Ident(name) => {
            if let Some(b) = builtins.get(name) {
                match b {
                    Builtin::Const(val) => val.clone(),
                    Builtin::Func(_) => panic!("La función '{}' debe llamarse con argumentos", name),
                    Builtin::Proc(_) => panic!("El procedimiento '{}' debe llamarse con paréntesis", name),
                }
            } else {
                env.vars.get(name).cloned().unwrap_or(Value::Nil)
            }
        }
        Expr::Call { name, args } => {
            //println!("eval_expr brazo Expr::Call: entro");
            if let Some(proc) = env.procs.get(name).cloned() {
                // --- procedimiento definido por el usuario ---
                //println!("eval_expr: entrando al bloque de procedures");
                if proc.params.len() != args.len() {
                    panic!(
                        "El procedimiento '{}' esperaba {} argumentos, pero recibió {}",
                        name,
                        proc.params.len(),
                        args.len()
                    );
                }

                // crear un entorno local (vacío) que herede las funciones/procedimientos
                let mut local_env = RuntimeEnv {
                    vars: HashMap::new(),
                    procs: env.procs.clone(),
                    funcs: env.funcs.clone(),
                };

                // inicializar parámetros (evaluar argumentos en el entorno actual `env`)
                for (param, arg_expr) in proc.params.iter().zip(args.iter()) {
                    let val = eval_expr(arg_expr, env, builtins); // evaluar usando el env actual
                    // println!(
                    //     "eval_expr Insertando parámetro en el procedure {} = {:?}",
                    //     param.name, val
                    // );
                    local_env.vars.insert(param.name.clone(), val);
                }

                // --- inicializar variables locales con valores por defecto ---
                for local in &proc.locals {
                    let default_val = match &local.ty {
                        VarType::Integer => Value::Integer(0),
                        VarType::Real => Value::Real(0.0),
                        VarType::Boolean => Value::Boolean(false),
                        VarType::Str => Value::Str(String::new()),
                        VarType::Nil => Value::Nil,
                    };
                    // println!(
                    //     "eval_expr Insertando variable local en procedure {} = {:?}",
                    //     local.name, default_val
                    // );
                    local_env.vars.insert(local.name.clone(), default_val);
                }

                // ejecutar el cuerpo del procedimiento en el entorno local
                for stmt in &proc.body {
                    // ignoramos posibles ReturnError: procedimientos no deberían retornar valores
                    let _ = execute_stmt(stmt, &mut local_env, builtins);
                }

                Value::Nil
            } else if let Some(func) = env.funcs.get(name).cloned() {
                // --- funcion definida por el usuario ---
                //println!("eval_expr: entrando al bloque de función");

                // --- crear un entorno local para la función ---
                let mut local_env = RuntimeEnv {
                    vars: HashMap::new(), // 🔑 empieza vacío
                    procs: env.procs.clone(),
                    funcs: env.funcs.clone(),
                };

                // --- inicializar parámetros con los valores evaluados ---
                for (param, arg_expr) in func.params.iter().zip(args.iter()) {
                    let val = eval_expr(arg_expr, env, builtins);
                    //println!("eval_expr Insertando parámetro {} = {:?}", param.name, val);
                    local_env.vars.insert(param.name.clone(), val);
                }

                // --- inicializar variables locales con valores por defecto ---
                for local in &func.locals {
                    let default_val = match &local.ty {
                        VarType::Integer => Value::Integer(0),
                        VarType::Real => Value::Real(0.0),
                        VarType::Boolean => Value::Boolean(false),
                        VarType::Str => Value::Str(String::new()),
                        VarType::Nil => Value::Nil,
                    };
                    //println!("eval_expr Insertando variable local {} = {:?}", local.name, default_val);
                    local_env.vars.insert(local.name.clone(), default_val);
                }

                // --- ejecutar el cuerpo hasta encontrar un Return ---
                let mut result = Value::Nil;
                for stmt in &func.body {
                    match stmt {
                        Stmt::Return(expr) => {
                            result = eval_expr(expr, &mut local_env, builtins);

                            break;
                        }
                        // _ => execute_stmt(stmt, &mut local_env, builtins),
                        _ => match execute_stmt(stmt, &mut local_env, builtins) {
                            Ok(_) => continue,
                            Err(ReturnError(val)) => {
                                result = val;
                                break;
                            }
                        },
                    }
                }

                result
            } else if let Some(builtin) = builtins.get(name) {
                // --- builtin como writeln, write, read, etc. ---
                match builtin {
                    Builtin::Func(f) => {
                        let arg_values: Vec<Value> = args.iter().map(|a| eval_expr(a, env, builtins)).collect();
                        f(arg_values)
                    }
                    Builtin::Proc(p) => {
                        let arg_values: Vec<Value> = args.iter().map(|a| eval_expr(a, env, builtins)).collect();
                        p(arg_values);
                        Value::Nil
                    }
                    &Builtin::Const(_) => todo!(),
                }
            } else {
                panic!("eval_expr Función/procedimiento '{}' no definido", name);
            }
        }

        Expr::StringLiteral(s) => Value::Str(s.clone()),
        Expr::BooleanLiteral(b) => Value::Boolean(*b),
        Expr::BinaryOp { left, op, right } => {
            let l = eval_expr(left, env, builtins);
            let r = eval_expr(right, env, builtins);
            apply_op(l, op, r)
        }
        Expr::Nil => Value::Nil,
    }
}

pub fn execute_stmt(stmt: &Stmt, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) -> Result<(), ReturnError> {
    //println!("execute_stmt: entro");
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env, builtins)?;
            }
        }
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env, builtins);
            env.vars.insert(name.clone(), val);
        }
        Stmt::IfElse {
            cond,
            then_branch,
            else_branch,
        } => match eval_expr(cond, env, builtins) {
            Value::Boolean(b) => {
                if b {
                    execute_stmt(then_branch, env, builtins)?;
                } else if let Some(else_stmt) = else_branch {
                    execute_stmt(else_stmt, env, builtins)?;
                }
            }
            _ => panic!("La condición del if no es booleana"),
        },
        Stmt::Expr(expr) => {
            eval_expr(expr, env, builtins);
        }
        Stmt::ProcDecl {
            name,
            params,
            locals,
            body,
        } => {
            // params: Vec<(String, VarType)> desde el AST
            //println!("============");
            //println!("execute_stmt entro al match brazo Stmt::ProcDecl entro");
            //println!("name {}", name);
            //println!("params {:?}", params);
            //println!("body {:?}", body);

            env.procs.insert(
                name.clone(),
                Procedure {
                    name: name.clone(),
                    params: params
                        .iter()
                        .map(|(name, ty)| Param {
                            name: name.clone(),
                            ty: ty.clone(),
                        })
                        .collect(),
                    locals: locals
                        .iter()
                        .map(|(name, ty)| Param {
                            name: name.clone(),
                            ty: ty.clone(),
                        })
                        .collect(),
                    body: body.clone(),
                },
            );
        }

        Stmt::FuncDecl {
            name,
            params,
            locals,
            return_type,
            body,
        } => {
            //println!("============");
            //println!("execute_stmt entro al match");
            //println!("Stmt::FuncDecl entro");
            //println!("name {}", name);
            //println!("params {:?}", params);
            //println!("return_type {:?}", return_type);
            //println!("body {:?}", body);

            env.funcs.insert(
                name.clone(),
                Function {
                    name: name.clone(),
                    params: params
                        .iter()
                        .map(|(name, ty)| Param {
                            name: name.clone(),
                            ty: ty.clone(),
                        })
                        .collect(),
                    locals: locals
                        .iter()
                        .map(|(name, ty)| Param {
                            name: name.clone(),
                            ty: ty.clone(),
                        })
                        .collect(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                },
            );
        }

        Stmt::Return(expr) => {
            let val = eval_expr(expr, env, builtins);
            return Err(ReturnError(val));
        }
    }
    Ok(())
}
