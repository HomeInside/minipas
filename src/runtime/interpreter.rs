use crate::parser::ast::{Function, Procedure};
use crate::runtime::std_lib::builtins::Builtin;
use crate::{Expr, Op, Stmt, Value};
use std::collections::HashMap;

pub type Environment = HashMap<String, Value>;

//pub type ProcedureEnv = HashMap<String, Procedure>;

pub struct RuntimeEnv {
    pub vars: Environment,
    //pub procs: ProcedureEnv,
    pub funcs: HashMap<String, Function>, // 👈 nuevo
    pub procs: HashMap<String, Procedure>, // 👈 aquí
                                          //pub scopes: Vec<HashMap<String, Value>>, // si ya lo tienes para manejar scopes
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Self {
            vars: Environment::new(),
            procs: HashMap::new(),
            funcs: HashMap::new(),
            //scopes: HashMap::new(),
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
                //println!("entro al some proc");
                // --- procedimiento definido por el usuario ---
                for (param_name, arg_expr) in proc.params.iter().zip(args.iter()) {
                    let arg_value = eval_expr(arg_expr, env, builtins);
                    env.vars.insert(param_name.clone(), arg_value);
                }
                for stmt in &proc.body {
                    execute_stmt(stmt, env, builtins);
                }
                Value::Nil
            } else if let Some(_func) = env.funcs.get(name) {
                // --- función definida por el usuario ---
                todo!()
            } else if let Some(builtin) = builtins.get(name) {
                // --- builtin como writeln, write, read, etc. ---
                match builtin {
                    Builtin::Func(f) => {
                        let arg_values: Vec<Value> = args.iter().map(|a| eval_expr(a, env, builtins)).collect();
                        f(arg_values) // devuelve Value
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
            //println!("============");
            //println!("Stmt::ProcDecl entro");
            //println!("name {}", name);
            //println!("params {:?}", params);
            //println!("body {:?}", body);

            env.procs.insert(
                name.clone(),
                Procedure {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                },
            );
        } //Stmt::Return(_) => todo!(),
    }
}
