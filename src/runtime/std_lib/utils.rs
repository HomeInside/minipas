use crate::Value;

pub fn to_real(val: &Value) -> f64 {
    match val {
        Value::Integer(i) => *i as f64,
        Value::Real(f) => *f,
        _ => panic!("Función matemática: solo soporta números"),
    }
}
