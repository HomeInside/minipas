use super::core::read_source;
use crate::parser::{gen_ast, gen_pairs};
use std::path::PathBuf;

pub fn check_cmd(input: Option<PathBuf>) {
    println!("=========check==========");
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("check error: expected input file");
            std::process::exit(1);
        }
    };

    let input_str = input.to_string_lossy();
    if !input_str.ends_with(".mp") {
        eprintln!("minipas error: usa extensión '.mp'");
        std::process::exit(1);
    }

    println!("leyendo codigo fuente...");
    let src = read_source(&input);

    println!("generando pairs...");
    gen_pairs(&src);

    println!("generando AST...");
    gen_ast(&src);

    println!();
    println!("check OK.");
}
