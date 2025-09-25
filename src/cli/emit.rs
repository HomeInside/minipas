use super::core::read_source;
use crate::parser::{gen_ast, gen_pairs};
use std::path::PathBuf;

pub fn emit_cmd(input: Option<PathBuf>) {
    println!("=========emit==========");
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("emit error: expected input file");
            std::process::exit(1);
        }
    };

    let input_str = input.to_string_lossy();
    if !input_str.ends_with(".mp") {
        eprintln!("minipas error: extensión no válida, usa '.mp'");
        std::process::exit(1);
    }

    let src = read_source(&input);
    let pairs = gen_pairs(&src);
    println!();
    println!("Pairs");
    println!("{:#?}", pairs);

    let (ast, _) = gen_ast(&src);
    println!();
    println!("AST");
    println!("{:#?}", ast);
}
