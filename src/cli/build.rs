use super::binary::gen_bincode_from_ast;
use super::core::read_source;
use crate::parser::gen_ast;
use crate::print_info;
use std::path::PathBuf;

pub fn build_cmd(input: Option<PathBuf>, output: Option<PathBuf>) {
    println!("=========build==========");
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("build error: expected input file");
            std::process::exit(1);
        }
    };
    println!("file_input: {:?}", input);
    let output = output.unwrap_or_else(|| PathBuf::from("a.mpc"));
    let input_str = input.to_string_lossy(); // Convierte PathBuf a String

    if input_str.ends_with(".mp") {
        println!("listo para construir: {:?}", input_str);
    } else {
        println!("minipas error: extensión de archivo no valida.");
        println!("utilice '.mp', para las extensiones de archivo.");
        print_info();
        std::process::exit(1);
    }

    let output_str = output.to_string_lossy(); // Convierte PathBuf a String
    if output_str.ends_with(".mpc") {
        println!("listo para construir: {:?}", output_str);
    } else {
        println!("minipas error: extensión de archivo de salida no valida.");
        println!("utilice '.mpc', para las extensiones de archivo.");
        print_info();

        std::process::exit(1);
    }

    // println!("file_input: {:?}", input);
    //println!("output: {:?}", output);

    let src = read_source(&input);
    let (ast, _) = gen_ast(&src);

    // println!("AST: {:#?}", ast);
    println!("build OK");

    gen_bincode_from_ast(ast, &output);
}
