pub mod math;
use crate::Value;
use crate::runtime::std_lib::math::pow_fn;
use std::collections::HashMap;

pub type RuntimeFn = fn(Vec<Value>) -> Value;

#[derive(Clone)]
pub enum Builtin {
    Const(Value),
    Func(RuntimeFn), // devuelve un Value
    Proc(RuntimeFn), // puede devolver Value::Str("") o Value::Integer(0)     si quieres
}

pub fn default_builtins() -> HashMap<String, Builtin> {
    //let mut map = HashMap::new();
    let mut builtin: HashMap<String, Builtin> = HashMap::new();

    // Constantes
    builtin.insert("PI".to_string(), Builtin::Const(Value::Real(std::f64::consts::PI)));
    builtin.insert("TRUE".to_string(), Builtin::Const(Value::Boolean(true)));
    builtin.insert("FALSE".to_string(), Builtin::Const(Value::Boolean(false)));

    // Funciones matemáticas
    builtin.insert("pow".to_string(), Builtin::Func(pow_fn));

    // Procedimientos
    // builtin.insert("writeln".to_string(), Builtin::Proc(writeln_fn));

    builtin
}
