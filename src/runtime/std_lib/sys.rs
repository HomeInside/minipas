use crate::Value;
use rand::Rng;
use std::{env, process, thread, time};

pub fn random_fn(args: Vec<Value>) -> Value {
    let mut rng = rand::rng();

    match args.len() {
        0 => {
            // random sin argumentos -> 0 ≤ r < 1
            Value::Integer(rng.random_range(0..=1 as i64))
        }
        1 => {
            // random(N) -> 0 ≤ r < N
            match &args[0] {
                Value::Integer(i) => {
                    if *i <= 0 {
                        panic!("random(N) requiere N > 0");
                    } else if *i == 1 {
                        return Value::Integer(rng.random_range(0..=1));
                    }
                    Value::Integer(rng.random_range(0..=*i as i64))
                }
                Value::Real(f) => {
                    if *f <= 0.0 {
                        panic!("random(N) requiere N > 0");
                    } else if *f == 1.0 {
                        return Value::Real(rng.random_range(0.0..=1.0));
                    }
                    Value::Real(rng.random_range(0.0..=*f))
                }
                _ => panic!("random(N) solo acepta números"),
            }
        }
        _ => panic!("random() acepta como máximo 1 argumento"),
    }
}

pub fn sleep_fn(args: Vec<Value>) -> Value {
    match args.len() {
        0 => {
            // Let's sleep for 1 second:
            let millis = time::Duration::from_millis(1000);
            thread::sleep(millis);
        }
        1 => {
            // sleep(N) -> 0 ≤ r < N
            match &args[0] {
                Value::Integer(i) => {
                    if *i <= 0 {
                        panic!("sleep(N) requiere N > 0");
                    } else {
                        let millis = time::Duration::from_millis((*i * 1000) as u64);
                        thread::sleep(millis);
                    }
                }
                Value::Real(f) => {
                    if *f <= 0.0 {
                        panic!("sleep(N) requiere N > 0");
                    } else {
                        let millis = time::Duration::from_millis((*f * 1000.0) as u64);
                        thread::sleep(millis);
                    }
                }
                _ => panic!("sleep(N) solo acepta valores enteros"),
            }
        }
        _ => panic!("sleep() acepta como máximo 1 argumento"),
    }
    // devolver algo para cumplir con el tipo
    // Value::None
    Value::Integer(0)
}

pub fn platform_fn(_args: Vec<Value>) -> Value {
    Value::Str(env::consts::OS.to_string())
}

pub fn exit_fn(args: Vec<Value>) -> Value {
    let code = match args.len() {
        0 => 0, // si no hay argumento, salir con 0
        1 => match &args[0] {
            Value::Integer(i) => *i as i32,
            _ => panic!("exit(N) solo acepta valores enteros"),
        },
        _ => panic!("exit() acepta como máximo 1 argumento"),
    };

    process::exit(code)
}
pub fn version_fn(_args: Vec<Value>) -> Value {
    let version = env!("CARGO_PKG_VERSION"); // versión desde Cargo.toml
    let build_time = option_env!("BUILD_TIME").unwrap_or("unknown");
    let compiler = option_env!("RUSTC_VERSION").unwrap_or("rustc");
    let target = env::consts::OS;

    let formatted = format!("{} (MiniPas build, {}, {}) [{}]", version, build_time, target, compiler);

    Value::Str(formatted)
}

/*
pub fn stderr_write(msg: &str) {
    eprint!("{}", msg);
}
*/
