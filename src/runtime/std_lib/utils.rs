use crate::Value;
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
