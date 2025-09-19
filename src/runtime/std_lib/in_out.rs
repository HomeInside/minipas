use super::utils::{get_input_default, unescape_string};
use crate::parser::ast::Value;

pub fn writeln_fn(args: Vec<Value>) -> Value {
    let parts: Vec<String> = args
        .into_iter()
        .map(|v| unescape_string(&v.to_string()))
        //.map(|v| v.to_string())
        .collect();

    println!("{}", parts.join(" "));
    Value::Nil
}

pub fn write_fn(args: Vec<Value>) -> Value {
    let parts: Vec<String> = args.into_iter().map(|v| unescape_string(&v.to_string())).collect();

    print!("{}", parts.join(" "));
    Value::Nil
}

pub fn readln_fn(args: Vec<Value>) -> Value {
    if !args.is_empty() {
        panic!("readln() no acepta argumentos");
    }

    let value: String = get_input_default("> ");
    Value::Str(value)
    //let value: u32 = get_input_default("> ");
    //Value::Integer(value.into())
}

pub fn format_fn(args: Vec<Value>) -> Value {
    if args.is_empty() {
        return Value::Str("".to_string());
    }

    let template = match &args[0] {
        Value::Str(s) => s.clone(),
        _ => return Value::Str("".to_string()), // o RuntimeError
    };

    let mut result = String::new();
    let mut arg_index = 1;

    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' && chars.peek() == Some(&'}') {
            // Caso {} simple
            chars.next(); // consume '}'
            if arg_index < args.len() {
                result.push_str(&args[arg_index].to_string());
                arg_index += 1;
            }
        } else if ch == '{' && chars.peek() == Some(&':') {
            // Caso {:spec}
            chars.next(); // consume ':'
            let mut spec = String::new();
            while let Some(&c) = chars.peek() {
                if c == '}' {
                    chars.next(); // consume '}'
                    break;
                }
                spec.push(c);
                chars.next();
            }
            if arg_index < args.len() {
                let val = &args[arg_index];
                let formatted = match (val, spec.as_str()) {
                    (Value::Real(r), s) => {
                        if s.ends_with('f') {
                            if let Ok(n) = s.trim_end_matches('f').parse::<usize>() {
                                format!("{:.*}", n, r)
                            } else {
                                r.to_string()
                            }
                        } else if let Ok(n) = s.parse::<usize>() {
                            format!("{:.*}", n, r)
                        } else {
                            r.to_string()
                        }
                    }
                    _ => val.to_string(),
                };
                result.push_str(&formatted);
                arg_index += 1;
            }
        } else {
            result.push(ch);
        }
    }

    Value::Str(result)
}
