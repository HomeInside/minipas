use crate::Value;

pub fn pow_fn(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("pow necesita exactamente 2 argumentos");
    }

    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Value::Real((*a as f64).powf(*b as f64)),
        (Value::Integer(a), Value::Real(b)) => Value::Real((*a as f64).powf(*b)),
        (Value::Real(a), Value::Integer(b)) => Value::Real(a.powf(*b as f64)),
        (Value::Real(a), Value::Real(b)) => Value::Real(a.powf(*b)),
        _ => panic!("pow solo acepta enteros o reales(flotantes)"),
    }
}
