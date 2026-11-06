use crate::parser::ast::{Function, Param, Procedure, VarType};
use crate::runtime::std_lib::builtins::Builtin;
use std::collections::HashMap;
pub type Environment = HashMap<String, Value>;
use super::operators::apply_op;
use crate::parser::ast::{Expr, ForDir, Stmt, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeFlow {
    Return(Value),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    pub vars: Environment,
    pub funcs: HashMap<String, Function>,
    pub procs: HashMap<String, Procedure>,
    pub constants: HashMap<String, Value>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Self {
            vars: Environment::new(),
            procs: HashMap::new(),
            funcs: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    // TO FIX
    // obtener el valor de una variable, falla si no existe
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Value {
        self.vars
            .get(name)
            .unwrap_or_else(|| panic!("Variable '{}' no declarada o no inicializada", name))
            .clone()
    }

    // TO FIX
    // Asignar valor a variable, falla si no existe
    #[allow(dead_code)]
    pub fn set(&mut self, name: &str, val: Value) {
        assert!(self.vars.contains_key(name), "Variable '{}' no declarada", name);
        self.vars.insert(name.to_string(), val);
    }

    // Declarar una variable nueva (por ejemplo en var_decl)
    pub fn declare(&mut self, name: &str, val: VarType) {
        assert!(!self.vars.contains_key(name), "Variable '{}' ya declarada", name);

        let default_val = match val {
            VarType::Integer => Value::Integer(0),
            VarType::Real => Value::Real(0.0),
            VarType::Str => Value::Str(String::new()),
            VarType::Boolean => Value::Boolean(false),
            VarType::Byte => Value::Byte(0),
            VarType::Nil => Value::Nil,
        };

        self.vars.insert(name.to_string(), default_val);
    }
}

pub fn eval_expr(expr: &Expr, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) -> Value {
    //println!("eval_expr entro a la funcion");
    //println!("eval_expr expr: {:?}", expr);
    match expr {
        #[allow(clippy::cast_lossless)]
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                Value::Integer(*n as i64)
            } else {
                Value::Real(*n)
            }
        }
        //Expr::Number(n) => Value::Integer(*n),
        Expr::Real(f) => Value::Real(*f),

        Expr::Ident(name) => {
            // constantes definidas por el usuario
            if let Some(val) = env.constants.get(name) {
                val.clone()
            // constantes y func/proc del sistema (builtins)
            } else if let Some(b) = builtins.get(name) {
                match b {
                    Builtin::Const(val) => val.clone(),
                    Builtin::Func(_) => panic!("La función '{}' debe llamarse con argumentos", name),
                    Builtin::Proc(_) => panic!("El procedimiento '{}' debe llamarse con paréntesis", name),
                }
            // variables de entorno
            } else {
                env.vars.get(name).cloned().unwrap_or(Value::Nil)
            }
        }
        Expr::Call { name, args } => {
            //println!("eval_expr brazo Expr::Call: entro");
            if let Some(proc) = env.procs.get(name).cloned() {
                // --- procedimiento definido por el usuario ---
                //println!("eval_expr: entrando al bloque de procedures");
                assert!(
                    proc.params.len() == args.len(),
                    "El procedimiento '{}' esperaba {} argumentos, pero recibió {}",
                    name,
                    proc.params.len(),
                    args.len()
                );

                // crear un entorno local (vacío) que herede las funciones/procedimientos
                let mut local_env = RuntimeEnv {
                    vars: HashMap::new(),
                    procs: env.procs.clone(),
                    funcs: env.funcs.clone(),
                    constants: env.constants.clone(),
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
                        VarType::Str => Value::Str(String::new()),
                        VarType::Boolean => Value::Boolean(false),
                        VarType::Byte => Value::Byte(0),
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
                    match execute_stmt(stmt, &mut local_env, builtins) {
                        Ok(()) => {}
                        Err(RuntimeFlow::Return(_)) => {
                            panic!("'return' no permitido en procedimientos");
                        }
                        Err(RuntimeFlow::Break | RuntimeFlow::Continue) => {
                            panic!("'break' o 'continue' usados fuera de un ciclo");
                        }
                    }
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
                    constants: env.constants.clone(),
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
                        VarType::Str => Value::Str(String::new()),
                        VarType::Boolean => Value::Boolean(false),
                        VarType::Byte => Value::Byte(0),
                        VarType::Nil => Value::Nil,
                    };
                    //println!("eval_expr Insertando variable local {} = {:?}", local.name, default_val);
                    local_env.vars.insert(local.name.clone(), default_val);
                }

                // --- ejecutar el cuerpo hasta encontrar un Return ---
                let mut result = Value::Nil;

                for stmt in &func.body {
                    match execute_stmt(stmt, &mut local_env, builtins) {
                        Ok(()) => {}
                        Err(RuntimeFlow::Return(val)) => {
                            result = val;
                            break;
                        }
                        Err(RuntimeFlow::Break | RuntimeFlow::Continue) => {
                            panic!("'break' o 'continue' usados fuera de un ciclo");
                        }
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
        Expr::MethodCall { target, method, args } => {
            assert!(
                builtins.contains_key(method),
                "Método '{}' no está registrado como builtin",
                method
            );

            // 1. evalúa el objeto y los argumentos
            let recv_val = eval_expr(target, env, builtins);
            let mut arg_values: Vec<Value> = args.iter().map(|a| eval_expr(a, env, builtins)).collect();

            // 2. inserta el "receptor" como primer argumento (igual que self)
            // Ej: "hola".upper()  →  upper("hola")
            arg_values.insert(0, recv_val.clone());

            // 3. busca el builtin correspondiente
            if let Some(builtin) = builtins.get(method) {
                match builtin {
                    Builtin::Func(f) => f(arg_values),
                    Builtin::Proc(p) => {
                        p(arg_values);
                        Value::Nil
                    }
                    Builtin::Const(_) => panic!("No se puede invocar una constante como método"),
                }
            } else {
                panic!("Método '{}' no definido (ni como builtin)", method);
            }
        }
    }
}

/// Ejecuta cada sentencia del programa.
///
/// # Arguments
///
/// * `stmt: &Stmt` - cada sentencia individual
///   del programa.
/// * `env: &mut RuntimeEnv` - el entorno donde se
///   encuentra la tabla se simbolos
/// * `builtins: &HashMap<String, Builtin>` - la biblioteca
///   estandar
///
/// # Returns
///
/// un Result `Result<(), RuntimeFlow>`
///
/// # Examples
///
/// ```rust,no_run,ignore
/// use crate::runtime::interpreter::execute_stmt;
/// ...
///
/// execute_stmt(stmt, &mut env, &BUILTINS) {
/// ```
///
pub fn execute_stmt(stmt: &Stmt, env: &mut RuntimeEnv, builtins: &HashMap<String, Builtin>) -> Result<(), RuntimeFlow> {
    //println!("execute_stmt: entro");
    match stmt {
        Stmt::Block(stmts) => {
            for s in stmts {
                execute_stmt(s, env, builtins)?;
            }
        }
        Stmt::Assign(name, expr) => {
            let rhs = eval_expr(expr, env, builtins);

            assert!(
                !env.constants.contains_key(name),
                "Error en tiempo de ejecución: intento de modificar la constante '{}'",
                name
            );

            let current = env
                .vars
                .get(name)
                .unwrap_or_else(|| panic!("Variable '{}' no declarada", name));

            // convertir según el tipo actual
            let coerced = match current {
                #[allow(clippy::cast_lossless)]
                Value::Integer(_) => match rhs {
                    Value::Integer(v) => Value::Integer(v),
                    Value::Byte(v) => Value::Integer(v as i64),
                    _ => panic!("{:?} cannot be assigned to Integer", rhs),
                },
                #[allow(clippy::cast_lossless)]
                Value::Real(_) => match rhs {
                    Value::Integer(v) => Value::Real(v as f64),
                    Value::Byte(v) => Value::Real(v as f64),
                    Value::Real(v) => Value::Real(v),
                    _ => panic!("{:?} cannot be assigned to Real", rhs),
                },
                Value::Str(_) => match rhs {
                    Value::Str(s) => Value::Str(s),
                    _ => panic!("{:?} cannot be assigned to String", rhs),
                },
                Value::Boolean(_) => match rhs {
                    Value::Boolean(b) => Value::Boolean(b),
                    _ => panic!("{:?} cannot be assigned to Boolean", rhs),
                },
                Value::Byte(_) => match rhs {
                    Value::Integer(v) => {
                        if v >= u8::MIN.into() && v <= u8::MAX.into() {
                            if let Ok(x) = u8::try_from(v) {
                                Value::Byte(x)
                            } else {
                                panic!("Out of range for Byte: {}", v);
                            }
                        } else {
                            panic!("Out of range for Byte: {}", v);
                        }
                    }
                    Value::Byte(v) => Value::Byte(v),
                    _ => panic!("{:?} cannot be assigned to Byte", rhs),
                },
                Value::Nil => rhs,
            };

            env.vars.insert(name.clone(), coerced);
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
            return Err(RuntimeFlow::Return(val));
        }
        Stmt::For {
            var,
            start,
            end,
            direction,
            body,
        } => {
            // println!("============");
            // println!("execute_stmt entro al match brazo Stmt::For");
            // println!("var {:?}", var);
            // println!("start {:?}", start);
            // println!("end {:?}", end);
            // println!("direction {:?}", direction);
            // println!("body {:?}", body);

            let mut i = eval_expr(start, env, builtins).as_int();
            let end_val = eval_expr(end, env, builtins).as_int();

            match direction {
                ForDir::To => {
                    while i <= end_val {
                        env.vars.insert(var.clone(), Value::Integer(i));

                        match execute_stmt(body, env, builtins) {
                            Ok(()) => {}
                            Err(RuntimeFlow::Continue) => {
                                i += 1;
                                continue;
                            }
                            Err(RuntimeFlow::Break) => break,
                            Err(RuntimeFlow::Return(v)) => return Err(RuntimeFlow::Return(v)),
                        }

                        i += 1;
                    }
                }
                ForDir::DownTo => {
                    while i >= end_val {
                        env.vars.insert(var.clone(), Value::Integer(i));

                        match execute_stmt(body, env, builtins) {
                            Ok(()) => {}
                            Err(RuntimeFlow::Continue) => {
                                i -= 1;
                                continue;
                            }
                            Err(RuntimeFlow::Break) => break,
                            Err(RuntimeFlow::Return(v)) => return Err(RuntimeFlow::Return(v)),
                        }

                        i -= 1;
                    }
                }
            }
        }
        Stmt::While(ws) => {
            //println!("============");
            //println!("execute_stmt entro al match brazo Stmt::While");
            //println!("ws {:?}", ws);

            while eval_expr(&ws.condition, env, builtins).as_bool() {
                match execute_stmt(&ws.body, env, builtins) {
                    Ok(()) => {}
                    Err(RuntimeFlow::Continue) => continue,
                    Err(RuntimeFlow::Break) => break,
                    Err(RuntimeFlow::Return(v)) => return Err(RuntimeFlow::Return(v)),
                }
            }
        }
        Stmt::Repeat { body, condition } => {
            //println!("============");
            //println!("execute_stmt entro al match brazo Stmt::Repeat");
            //println!("body {:?}", body);
            //println!("condition {:?}", condition);

            loop {
                // ejecutar cuerpo
                for s in body {
                    match execute_stmt(s, env, builtins) {
                        Ok(()) => {}
                        // `continue` termina el bucle y reevalúa la condición
                        // saltando al `cond_val`
                        Err(RuntimeFlow::Continue) => break,
                        // `break` sale del bucle completo, termina el bucle de repeat
                        Err(RuntimeFlow::Break) => return Ok(()),
                        Err(RuntimeFlow::Return(v)) => return Err(RuntimeFlow::Return(v)),
                    }
                }

                // evaluar condición al final
                let cond_val = eval_expr(condition, env, builtins).as_bool();
                if cond_val {
                    break;
                }
            }
        }
        Stmt::Break => return Err(RuntimeFlow::Break),
        Stmt::Continue => return Err(RuntimeFlow::Continue),
    }
    Ok(())
}
