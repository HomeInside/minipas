use crate::parser::ast::Value;

pub fn to_int_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("to_int() espera 1 argumento");
    }
    match &args[0] {
        Value::Str(s) => Value::Integer(
            s.parse::<i64>()
                .unwrap_or_else(|_| panic!("No se pudo convertir a int: '{}'", s)),
        ),
        Value::Real(r) => Value::Integer(*r as i64),
        Value::Integer(i) => Value::Integer(*i),
        _ => panic!("Tipo no soportado en to_int"),
    }
}

pub fn to_real_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("to_real() espera 1 argumento");
    }
    match &args[0] {
        Value::Str(s) => Value::Real(s.parse::<f64>().expect("No se pudo convertir a real")),
        Value::Integer(i) => Value::Real(*i as f64),
        Value::Real(r) => Value::Real(*r),
        _ => panic!("Tipo no soportado en to_real"),
    }
}

pub fn to_str_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("to_str() espera 1 argumento");
    }
    Value::Str(args[0].to_string()) // suponiendo que implementaste Display para Value
}
