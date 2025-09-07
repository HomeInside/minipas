use crate::{Expr, Op, Stmt};
use std::collections::HashMap;

type Environment = HashMap<String, f64>;

fn eval_expr(expr: &Expr, env: &mut HashMap<String, f64>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) => *env.get(name).unwrap_or(&0.0),
        Expr::BinaryOp { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
                Op::Greater => (l > r) as i64 as f64,
                Op::Less => (l < r) as i64 as f64,
                Op::GreaterEq => (l >= r) as i64 as f64,
                Op::LessEq => (l <= r) as i64 as f64,
                Op::Equal => (l == r) as i64 as f64,
                Op::NotEqual => (l != r) as i64 as f64,
            }
        }
        Expr::StringLiteral(_) => panic!("No se puede evaluar un StringLiteral como número"),
    }
}

pub fn execute_stmt(stmt: &Stmt, env: &mut Environment) {
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env);
            }
        }

        /*Stmt::VarDecl(vars) => {
            for v in vars {
                env.insert(v.clone(), 0.0);
            }
        }*/
        Stmt::Assign(name, expr) => {
            let val = eval_expr(expr, env);
            env.insert(name.clone(), val);
        }
        Stmt::WritelnList(exprs) => {
            let mut outputs = Vec::new();
            for e in exprs {
                match e {
                    Expr::StringLiteral(s) => outputs.push(s.clone()),
                    _ => outputs.push(eval_expr(e, env).to_string()),
                }
            }
            // N decimales
            /*let outputs: Vec<String> = exprs.iter().map(|e| {
                match e {
                    Expr::StringLiteral(s) => s.clone(),
                    _ => format!("{:.3}", eval_expr(e, env))
                }
            }).collect();*/

            println!("{}", outputs.join(" "));
        }

        Stmt::IfElse {
            cond,
            then_branch,
            else_branch,
        } => {
            if eval_expr(cond, env) != 0.0 {
                execute_stmt(then_branch, env);
            } else if let Some(else_stmt) = else_branch {
                execute_stmt(else_stmt, env);
            }
        }
    }
}
