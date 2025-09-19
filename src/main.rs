use pest::Parser;
use pest_derive::Parser;
use std::env;
use std::fs;

mod parser;
mod runtime;

use parser::program::parse_program;
use runtime::interpreter::{RuntimeEnv, execute_stmt};
use runtime::std_lib::builtins::BUILTINS;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MiniPasParser;

fn main() {
    println!("welcome to minipas v{}", env!("CARGO_PKG_VERSION"));
    // Tomar el argumento de la línea de comandos (ej: `cargo run ejemplo.mp`)
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

    // Leer el archivo .mp
    let input = fs::read_to_string(filename).unwrap_or_else(|_| panic!("No se pudo leer el archivo {}", filename));

    // Parsear
    let pairs = MiniPasParser::parse(Rule::program, &input).unwrap_or_else(|e| panic!("Error de parseo: {}", e));

    let mut env = RuntimeEnv::new();

    let (program, _) = parse_program(pairs);

    // Depuración: imprime el AST
    //println!("AST: {:#?}", program);

    for stmt in &program {
        let _ = execute_stmt(stmt, &mut env, &BUILTINS);
    }

    //println!("ENV: {:#?}", env);
}
