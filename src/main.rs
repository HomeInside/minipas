use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::env;
use std::fs;

mod parser;
mod runtime;

use parser::ast::{Expr, Op, Stmt, Value, VarType};
use parser::parser::*;
use runtime::interpreter::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PascalParser;

type Environment = HashMap<String, Value>;

fn default_constants() -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("PI".to_string(), Value::Real(std::f64::consts::PI));
    map.insert("TRUE".to_string(), Value::Boolean(true));
    map.insert("FALSE".to_string(), Value::Boolean(false));
    map
}

fn main() {
    println!("welcome to minipas v{}", env!("CARGO_PKG_VERSION"));
    // Tomar el argumento de la línea de comandos (ej: `cargo run ejemplo.pas`)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <archivo.mp>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    println!("cargando archivo: '{}'", filename);

    if !filename.ends_with(".mp") {
        println!("minipas error: extensión de archivo no valida.");
        println!("utilice `.mp`, para las extensiones de archivo.");
        std::process::exit(1);
    }

    // Leer el archivo .pas
    let input = fs::read_to_string(filename).unwrap_or_else(|_| panic!("No se pudo leer el archivo {}", filename));

    // Parsear
    let pairs = PascalParser::parse(Rule::program, &input).unwrap_or_else(|e| panic!("Error de parseo: {}", e));

    //let program = parse_program(pairs);
    let (program, _) = parse_program(pairs);

    // Depuración: imprime el AST
    // println!("AST: {:#?}", program);

    let mut env = Environment::new();
    let consts = default_constants();

    for stmt in &program {
        execute_stmt(stmt, &mut env, &consts);
    }

    // println!("ENV: {:#?}", env);
}
