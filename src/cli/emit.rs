use super::core::{read_source, save_ast_as_text, save_pairs_as_text};
use crate::parser::{gen_ast, gen_pairs};
use std::path::PathBuf;

pub fn emit_cmd(input: Option<PathBuf>) {
    //println!("=========emit==========");
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

    let (ast, _) = gen_ast(&src);
    println!();

    let stem = input.file_stem().unwrap().to_string_lossy();
    let pairs_file = input.with_file_name(format!("{}.mpp", stem)); // mp pairs file
    let ast_file = input.with_file_name(format!("{}.mpa", stem)); // mp ast file

    println!("generating Pairs file: {:?}", pairs_file);
    save_pairs_as_text(&pairs, &pairs_file);

    println!("generating AST file (text): {:?}", ast_file);
    save_ast_as_text(ast, &ast_file);

    println!();
    println!("OK.");
}
