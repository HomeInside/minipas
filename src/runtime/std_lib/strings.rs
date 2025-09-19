use crate::Value;

pub fn len_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("len()/length() necesita 1 argumento");
    }
    Value::Integer(args[0].to_string().len() as i64)
}

pub fn upper_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("upper() necesita 1 argumento");
    }
    Value::Str(args[0].to_string().to_uppercase())
}

pub fn lower_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("lower() necesita 1 argumento");
    }
    Value::Str(args[0].to_string().to_lowercase())
}

pub fn trim_fn(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("trim() necesita 1 argumento");
    }
    let str_trim = args[0].to_string().trim().to_string();
    Value::Str(str_trim)
}

pub fn concat_fn(args: Vec<Value>) -> Value {
    let parts: Vec<String> = args.into_iter().map(|v| v.to_string()).collect();

    let str_cnct = parts.join(" ");
    Value::Str(str_cnct)
}
