use crate::runtime::std_lib::Builtin;
use crate::{Expr, Op, Stmt, Value};
use std::collections::HashMap;

type Environment = HashMap<String, Value>;

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

fn eval_expr(expr: &Expr, env: &mut Environment, builtins: &HashMap<String, Builtin>) -> Value {
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
                env.get(name).cloned().unwrap_or(Value::Real(0.0))
            }
        }
        Expr::Call { name, args } => {
            let arg_vals: Vec<Value> = args.iter().map(|a| eval_expr(a, env, builtins)).collect();
            if let Some(b) = builtins.get(name) {
                match b {
                    Builtin::Func(f) => f(arg_vals),
                    Builtin::Proc(f) => f(arg_vals),
                    Builtin::Const(_) => panic!("'{}' no es una función", name),
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

pub fn execute_stmt(stmt: &Stmt, env: &mut Environment, builtins: &HashMap<String, Builtin>) {
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env, builtins);
            }
        }
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env, builtins);
            env.insert(name.clone(), val);
        }
        Stmt::WritelnList(exprs) => {
            let outputs: Vec<String> = exprs
                .iter()
                .map(|e| eval_expr(e, env, builtins).to_string_value())
                .collect();
            println!("{}", outputs.join(" "));
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
    }
}
