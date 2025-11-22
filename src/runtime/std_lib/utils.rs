use crate::parser::ast::Value;
use std::io::{self, Write};

pub fn to_real(val: &Value) -> f64 {
    match val {
        Value::Integer(i) => *i as f64,
        Value::Real(f) => *f,
        _ => panic!("Función matemática: solo soporta números"),
    }
}

pub fn get_input_default<U: std::str::FromStr>(prompt: &str) -> U {
    loop {
        let mut line = String::new();

        // Reads the input from STDIN and places it in the String named line.
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin().read_line(&mut line).expect("Failed to read input.");

        let input_trimmed = line.trim();
        // Convert to another type.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input = match input_trimmed.parse::<U>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => continue,
        };

        return input;
    }
}

pub fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('n') => {
                    result.push('\n');
                    chars.next();
                }
                Some('r') => {
                    result.push('\n');
                    chars.next();
                }
                Some('t') => {
                    result.push('\t');
                    chars.next();
                }
                Some('"') => {
                    result.push('"');
                    chars.next();
                }
                Some('\\') => {
                    result.push('\\');
                    chars.next();
                }
                Some(other) => {
                    // cualquier otro carácter después de \ se toma literal
                    result.push(*other);
                    chars.next();
                }
                None => result.push('\\'), // \ al final de la cadena
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn expect_arg_count(fn_name: &str, args: &[Value], expected: usize, is_method: bool) {
    let user_count = if is_method { args.len() - 1 } else { args.len() };
    if user_count != expected {
        let label = if is_method {
            format!("{}() no toma argumentos (se dio {})", fn_name, user_count)
        } else {
            format!("{}() espera {} argumento(s), recibió {}", fn_name, expected, user_count)
        };
        panic!("{}", label);
    }
}
