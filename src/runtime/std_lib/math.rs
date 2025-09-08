use crate::Value;

pub fn pow_fn(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("pow() necesita exactamente 2 argumentos");
    }

    let base = match &args[0] {
        Value::Integer(i) => *i as f64,
        Value::Real(f) => *f,
        _ => panic!("pow() base number: solo soporta números"),
    };
    let exp = match &args[1] {
        Value::Integer(i) => *i as f64,
        Value::Real(f) => *f,
        _ => panic!("pow() exp number: solo soporta números"),
    };

    let res = base.powf(exp);
    if res.fract() == 0.0 {
        Value::Integer(res as i64)
    } else {
        Value::Real(res)
    }
}
