use super::core::read_source;
use crate::parser::{gen_ast, gen_pairs};
use crate::print_info;
use std::path::PathBuf;

pub fn check_cmd(input: Option<PathBuf>) {
    //println!("=========check==========");
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("check error: expected input file");
            std::process::exit(1);
        }
    };

    let input_str = input.to_string_lossy();
    if !input_str.ends_with(".mp") {
        println!("minipas error: extensión de archivo de entrada, no valido.");
        println!("utilice '.mp', para las extensiones de archivo.");
        print_info();
        std::process::exit(1);
    }

    //println!("leyendo codigo fuente...");
    let src = read_source(&input);

    println!("validando pairs...");
    gen_pairs(&src);

    println!("validando AST...");
    gen_ast(&src);

    println!();
    println!("OK.");
}
