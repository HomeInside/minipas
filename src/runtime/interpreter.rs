use crate::parser::ast::{Function, Procedure};
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

fn eval_expr(expr: &Expr, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) -> Value {
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
                env.vars.get(name).cloned().unwrap_or(Value::Real(0.0))
            }
        }
        Expr::Call { name, args } => {
            if let Some(proc) = env.procs.get(name).cloned() {
                // --- procedimiento definido por el usuario ---
                //println!("entro al some proc");
                for (param_name, arg_expr) in proc.params.iter().zip(args.iter()) {
                    let arg_value = eval_expr(arg_expr, env, builtins);
                    env.vars.insert(param_name.clone(), arg_value);
                }
                for stmt in &proc.body {
                    execute_stmt(stmt, env, builtins);
                }
                Value::Nil
            } else if let Some(func) = env.funcs.get(name).cloned() {
                // --- función definida por el usuario ---
                // Crear un entorno local (scope)
                println!("eval_expr: entro al some func de Expr::Call");
                let mut local_env = env.clone();
                /*let mut local_env = RuntimeEnv {
                    vars: std::collections::HashMap::new(),
                    procs: env.procs.clone(),
                    funcs: env.funcs.clone(),
                };*/

                for (param_name, arg_expr) in func.params.iter().zip(args.iter()) {
                    let arg_value = eval_expr(arg_expr, env, builtins);
                    local_env.vars.insert(param_name.clone(), arg_value);
                }

                // Ejecutar el cuerpo hasta encontrar Return
                let mut result = Value::Nil;
                for stmt in &func.body {
                    match stmt {
                        Stmt::Return(expr) => {
                            result = eval_expr(expr, &mut local_env, builtins);
                            break;
                        }
                        _ => execute_stmt(stmt, &mut local_env, builtins),
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
                panic!("Función/procedimiento '{}' no definido", name);
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

pub fn execute_stmt(stmt: &Stmt, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) {
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env, builtins);
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
                    execute_stmt(then_branch, env, builtins);
                } else if let Some(else_stmt) = else_branch {
                    execute_stmt(else_stmt, env, builtins);
                }
            }
            _ => panic!("La condición del if no es booleana"),
        },
        Stmt::Expr(expr) => {
            eval_expr(expr, env, builtins);
        }
        Stmt::ProcDecl { name, params, body } => {
            env.procs.insert(
                name.clone(),
                Procedure {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                },
            );
        }
        Stmt::FuncDecl {
            name,
            params,
            return_type,
            body,
        } => {
            // 👈 NUEVO
            println!("============");
            println!("execute_stmt entro al match");
            println!("Stmt::FuncDecl entro");
            println!("name {}", name);
            println!("params {:?}", params);
            println!("return_type {:?}", return_type);
            println!("body {:?}", body);
            env.funcs.insert(
                name.clone(),
                Function {
                    name: name.clone(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                },
            );
        }
        Stmt::Return(_) => todo!(),
    }
}
