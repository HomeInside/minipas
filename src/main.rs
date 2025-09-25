use pest_derive::Parser;
mod cli;
mod parser;
mod runtime;
use cli::{
    build::build_cmd,
    check::check_cmd,
    core::{parse_input_file, parse_output_file},
    emit::emit_cmd,
    run::run_cmd,
};
use pico_args::Error;
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MiniPasParser;

#[derive(Debug)]
struct AppArgs {
    cmd: String,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

fn print_help() {
    println!(
        r#"
Usage: minipas [OPTIONS] [COMMANDS] <archivo.mp>

Options:
  -v, --version     Muestra la versión
  -o, --output      Especifica archivo de salida (por defecto: a.mpc)
  -h, --help        Muestra esta ayuda

Commands:
  build, b     Compila el archivo fuente y genera el AST en formato binario
  run, r       Ejecuta el archivo
  emit, e      Genera los Pairs y el AST
  check, c     Verifica el codigo fuente archivo sin ejecutarlo

EXAMPLES:
  minipas build hello_world.mp
  minipas run hello_world.mpc
  minipas build hello_world.mp -o a.mpc
  minipas emit hello_world.mp
  minipas check hello_world.mp
"#
    );
}

fn print_info() {
    println!();
    println!("try 'minipas --help' for more information");
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print_help();
        std::process::exit(0);
    }
    if pargs.contains(["-v", "--version"]) {
        std::process::exit(0);
    }

    // https://github.com/RazrFalcon/pico-args/issues/10
    let args = AppArgs {
        cmd: pargs.free_from_str()?,
        input: pargs.opt_free_from_os_str(parse_input_file)?,
        output: pargs.opt_value_from_os_str(["-o", "--output"], parse_output_file)?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();

    if !remaining.is_empty() {
        eprintln!("Warning: unknown arguments left: {:?}.", remaining);
        print_info();
        std::process::exit(1);
    }
    Ok(args)
}

fn main() {
    println!("minipas v.{}", env!("CARGO_PKG_VERSION"));
    match parse_args() {
        Ok(v) => match v.cmd.as_str() {
            "build" | "b" => build_cmd(v.input, v.output),
            "run" | "r" => run_cmd(v.input),
            "emit" | "e" => emit_cmd(v.input),
            "check" | "c" => check_cmd(v.input),
            _ => {
                eprintln!("Error: unknown command '{}'", v.cmd);
                print_info();
                std::process::exit(1);
            }
        },
        Err(e) => {
            match e {
                Error::MissingArgument => {
                    // eprintln!("Error: missing argument or command");
                    print_info();
                }
                Error::ArgumentParsingFailed { cause } => {
                    eprintln!("Error: argument parsing failed: {}", cause);
                    print_info();
                }
                Error::Utf8ArgumentParsingFailed { cause, value } => {
                    eprintln!("Error: UTF-8 arguments:");
                    eprintln!("cause: {}", cause);
                    eprintln!("details: {}", value);
                    print_info();
                }
                Error::OptionWithoutAValue(cause) => {
                    eprintln!("Error: option without a value: {:?}", cause);
                    print_info();
                }
                _ => {
                    eprintln!("Error: arguments error: {:?}", e);
                    print_info();
                }
            }
            //print_info();
            std::process::exit(1);
        }
    }
}
