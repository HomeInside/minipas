use super::core::read_source;
use crate::print_info;
use crate::runtime::{
    interpreter::{RuntimeEnv, execute_stmt},
    std_lib::builtins::BUILTINS,
};
use std::path::PathBuf;

use crate::parser::gen_ast;

pub fn run_cmd(input: Option<PathBuf>) {
    println!("=========run==========");
    //parsea y ejecuta el programa
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("run Error: expected input file");
            print_info();
            std::process::exit(1);
        }
    };

    //println!("file_input: {:?}", input);
    let input_str = input.to_string_lossy(); // Convierte PathBuf a String

    if input_str.ends_with(".mp") {
        println!("listo para parsear y ejecutar: {:?}", input_str);
    } else if input_str.ends_with(".mpc") {
        println!("listo para ejecutar: {:?}", input_str);
    } else {
        println!("minipas error: extensión de archivo no valida.");
        println!("utilice '.mp' ó '.mpc', para las extensiones de archivo.");
        print_info();
        std::process::exit(1);
    }

    let src = read_source(&input);
    let (ast, _) = gen_ast(&src);

    let mut env = RuntimeEnv::new();
    // ejecuta el programa
    for stmt in &ast {
        let _ = execute_stmt(stmt, &mut env, &BUILTINS);
    }
}
