use super::utils::to_real;
use crate::parser::ast::Value;

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

pub fn abs_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("abs() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).abs())
}

pub fn sin_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("sin() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).sin())
}

pub fn cos_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("cos() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).cos())
}

pub fn tan_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("tan() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).tan())
}

pub fn cot_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("cot() necesita 1 argumento");
    }
    let x = to_real(&args[0]);
    Value::Real(1.0 / x.tan())
}

pub fn asin_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("asin() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).asin())
}

pub fn acos_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("acos() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).acos())
}

pub fn atan_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("atan() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).atan())
}

pub fn exp_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("exp() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).exp())
}

pub fn ln_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("ln() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).ln())
}

pub fn log10_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("log10() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).log10())
}

pub fn sqrt_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("sqrt() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).sqrt())
}

pub fn floor_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("floor() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).floor())
}

pub fn ceil_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("ceil() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).ceil())
}

pub fn round_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("round() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).round())
}

pub fn trunc_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("trunc() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).trunc())
}

pub fn fract_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("fract() necesita 1 argumento");
    }
    Value::Real(to_real(&args[0]).fract())
}

pub fn max_fn(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("max() necesita 2 argumentos");
    }
    Value::Real(to_real(&args[0]).max(to_real(&args[1])))
}

pub fn min_fn(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("min() necesita 2 argumentos");
    }
    Value::Real(to_real(&args[0]).min(to_real(&args[1])))
}

pub fn sign_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("sign() necesita 1 argumento");
    }
    let r = to_real(&args[0]);
    Value::Integer(if r > 0.0 {
        1
    } else if r < 0.0 {
        -1
    } else {
        0
    })
}
