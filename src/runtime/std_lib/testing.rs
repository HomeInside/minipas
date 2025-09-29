use crate::parser::ast::Value;

pub fn assert_fn(args: Vec<Value>) {
    if args.len() != 1 {
        panic!("assert() necesita exactamente 1 argumento");
    }

    match &args[0] {
        Value::Boolean(true) => Value::Nil, // pasa
        Value::Boolean(false) => {
            panic!("Assertion failed: expected `true`, got `{}`\n", args[0]);
        }
        _ => panic!("assert() solo acepta un valor booleano"),
    };
}

pub fn assert_eq_fn(args: Vec<Value>) {
    if args.len() != 2 {
        panic!("assert_eq() necesita exactamente 2 argumentos");
    }

    let a = &args[0];
    let b = &args[1];

    let resp = match (a, b) {
        // mismos tipos
        (Value::Integer(x), Value::Integer(y)) => x == y,
        (Value::Real(x), Value::Real(y)) => (x - y).abs() < f64::EPSILON,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Boolean(x), Value::Boolean(y)) => x == y,
        (Value::Byte(x), Value::Byte(y)) => x == y,
        (Value::Nil, Value::Nil) => true,

        // validación entre tipos numéricos
        (Value::Integer(x), Value::Byte(y)) => *x == *y as i64,
        (Value::Byte(x), Value::Integer(y)) => *x as i64 == *y,
        (Value::Integer(x), Value::Real(y)) => (*x as f64 - *y).abs() < f64::EPSILON,
        (Value::Real(x), Value::Integer(y)) => (*x - *y as f64).abs() < f64::EPSILON,
        (Value::Byte(x), Value::Real(y)) => (*x as f64 - *y).abs() < f64::EPSILON,
        (Value::Real(x), Value::Byte(y)) => (*x - *y as f64).abs() < f64::EPSILON,

        _ => false,
    };

    if !resp {
        panic!("Assertion failed: expected left: `{}`, got right: `{}`", a, b);
    }
}

pub fn panic_fn(args: Vec<Value>) {
    if args.len() != 1 {
        panic!("panic() necesita exactamente 1 argumento");
    }
    panic!("runtime error: {}\n", args[0]);
}
