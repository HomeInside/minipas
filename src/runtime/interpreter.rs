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

fn eval_expr(expr: &Expr, env: &mut Environment, consts: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                Value::Integer(*n as i64)
            } else {
                Value::Real(*n)
            }
        }
        Expr::Ident(name) => {
            if let Some(c) = consts.get(name) {
                c.clone()
            } else {
                env.get(name).cloned().unwrap_or(Value::Real(0.0))
            }
        }
        Expr::StringLiteral(s) => Value::Str(s.clone()),
        Expr::BooleanLiteral(b) => Value::Boolean(*b),
        Expr::BinaryOp { left, op, right } => {
            let l = eval_expr(left, env, consts);
            let r = eval_expr(right, env, consts);
            apply_op(l, op, r)
        }
    }
}

pub fn execute_stmt(stmt: &Stmt, env: &mut Environment, consts: &HashMap<String, Value>) {
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env, consts);
            }
        }
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env, consts);
            env.insert(name.clone(), val);
        }
        Stmt::WritelnList(exprs) => {
            let outputs: Vec<String> = exprs
                .iter()
                .map(|e| eval_expr(e, env, consts).to_string_value())
                .collect();
            println!("{}", outputs.join(" "));
        }
        Stmt::IfElse {
            cond,
            then_branch,
            else_branch,
        } => match eval_expr(cond, env, consts) {
            Value::Boolean(b) => {
                if b {
                    execute_stmt(then_branch, env, consts);
                } else if let Some(else_stmt) = else_branch {
                    execute_stmt(else_stmt, env, consts);
                }
            }
            _ => panic!("La condición del if no es booleana"),
        },
    }
}
