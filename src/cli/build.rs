use super::binary::gen_bincode_from_ast;
use super::core::read_source;
use crate::parser::gen_ast;
use crate::print_info;
use std::path::PathBuf;

pub fn build_cmd(input: Option<PathBuf>, output: Option<PathBuf>) {
    //println!("=========build==========");
    let input = match input {
        Some(path) => path,
        None => {
            eprintln!("build error: expected input file");
            std::process::exit(1);
        }
    };
    let output = output.unwrap_or_else(|| PathBuf::from("a.mpc"));

    let input_str = input.to_string_lossy(); // Convierte PathBuf a String

    if !input_str.ends_with(".mp") {
        println!("minipas error: extensión de archivo de entrada no valida.");
        println!("utilice '.mp', para las extensiones de archivo.");
        print_info();
        std::process::exit(1);
    }

    let output_str = output.to_string_lossy(); // Convierte PathBuf a String
    if !output_str.ends_with(".mpc") {
        println!("minipas error: extensión de archivo de salida no valida.");
        println!("utilice '.mpc', para los archivos de salida.");
        print_info();

        std::process::exit(1);
    }

    println!("building {:?}...", input.display());
    let src = read_source(&input);
    let (ast, symbols) = gen_ast(&src);

    let stem = output.file_stem().unwrap().to_string_lossy();
    let output = output.with_file_name(format!("{}.mpc", stem)); // mp compile ast file
    println!("generating AST file (bin): {:?}", output.display());

    gen_bincode_from_ast(ast, symbols, &output);
    println!();
    println!("OK.");
}
