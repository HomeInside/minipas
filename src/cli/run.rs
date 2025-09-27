use super::binary::read_ast_from_bincode;
use super::core::read_source;
use crate::parser::ast::Stmt;
use crate::parser::gen_ast;
use crate::parser::symbol_table::SymbolTable;
use crate::print_info;
use crate::runtime::{
    interpreter::{RuntimeEnv, execute_stmt},
    std_lib::builtins::BUILTINS,
};
use std::path::PathBuf;

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

    if let Some(ext) = input.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();

        let mut sym_table: SymbolTable = SymbolTable::with_builtins();

        let ast: Vec<Stmt> = match ext_str.as_str() {
            "mp" => {
                println!("listo para parsear y ejecutar: {:?}", input.display());
                let src = read_source(&input);
                let (ast, sym_tbl) = gen_ast(&src);
                sym_table = sym_tbl;
                ast
            }
            "mpc" => {
                println!("listo para ejecutar: {:?}", input.display());
                read_ast_from_bincode(&input)
            }
            _ => {
                eprintln!("minipas error: extensión de archivo no válida.");
                eprintln!("run: utilice '.mp' ó '.mpc', para extensiones de archivo.");
                print_info();
                std::process::exit(1);
            }
        };

        // Crear el entorno y ejecutar el programa
        let mut env = RuntimeEnv::new();
        // Cargar la tabla de simbolos
        for (name, ty) in sym_table.global_vars() {
            env.declare(&name, ty);
        }

        for stmt in &ast {
            if let Err(e) = execute_stmt(stmt, &mut env, &BUILTINS) {
                eprintln!("minipas runtime error: {:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("minipas error: archivo sin extensión válida.");
        eprintln!("run: utilice '.mp' ó '.mpc', para extensiones de archivo.");
        print_info();
        std::process::exit(1);
    }
}
