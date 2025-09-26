use crate::parser::ast::Value;

pub fn assert_fn(args: Vec<Value>) -> Value {
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

    Value::Nil
}

pub fn assert_eq_fn(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("assert_eq() necesita exactamente 2 argumentos");
    }

    if args[0] != args[1] {
        panic!("Assertion failed: expected `{}`, got `{}`\n", args[0], args[1]);
    }

    Value::Nil
}

pub fn panic_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("panic() necesita exactamente 1 argumento");
    }
    panic!("runtime error: {}\n", args[0]);
}
