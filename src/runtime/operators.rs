use crate::parser::ast::{Op, Value};

pub fn apply_op(l: Value, op: &Op, r: Value) -> Value {
    //println!("apply_op entro");
    //println!("l: {}", l);
    //println!("r: {}", r);
    match (l, r) {
        (Value::Integer(li), Value::Integer(ri)) => match op {
            // TO TEST validar overflows
            Op::Add => match li.checked_add(ri) {
                Some(v) => Value::Integer(v),
                None => panic!("overflow en Integer + Integer: {} + {}", li, ri),
            },
            // TO TEST validar overflows
            Op::Sub => match li.checked_sub(ri) {
                Some(v) => Value::Integer(v),
                None => panic!("overflow en Integer - Integer: {} - {}", li, ri),
            },
            // TO TEST validar overflows
            Op::Mul => match li.checked_mul(ri) {
                Some(v) => Value::Integer(v),
                None => panic!("overflow en Integer * Integer: {} * {}", li, ri),
            },
            Op::Div => {
                if ri == 0 {
                    panic!("división por cero: {} / 0", li);
                }
                // la división entre enteros arroja como
                // resultado un real
                Value::Real(li as f64 / ri as f64)
            }
            Op::Idiv => {
                if ri == 0 {
                    panic!("división entera por cero: {} div 0", li);
                }
                Value::Integer(li / ri)
            }
            Op::Greater => Value::Boolean(li > ri),
            Op::Less => Value::Boolean(li < ri),
            Op::GreaterEq => Value::Boolean(li >= ri),
            Op::LessEq => Value::Boolean(li <= ri),
            Op::Equal => Value::Boolean(li == ri),
            Op::NotEqual => Value::Boolean(li != ri),
            Op::Mod => {
                if ri == 0 {
                    panic!("módulo por cero: {} mod 0", li);
                }
                //Value::Integer(li % ri)
                // resto matemático no negativo
                Value::Integer(li.rem_euclid(ri))
            }
        },

        (Value::Real(lf), Value::Real(rf)) => match op {
            Op::Add => Value::Real(lf + rf),
            Op::Sub => Value::Real(lf - rf),
            Op::Mul => Value::Real(lf * rf),
            Op::Div => {
                if rf == 0.0 {
                    panic!("división por cero: {} / 0.0", lf);
                }
                Value::Real(lf / rf)
            }
            Op::Idiv => {
                panic!("el operador 'div' solo se permite con Integer o Byte");
            }
            Op::Greater => Value::Boolean(lf > rf),
            Op::Less => Value::Boolean(lf < rf),
            Op::GreaterEq => Value::Boolean(lf >= rf),
            Op::LessEq => Value::Boolean(lf <= rf),
            Op::Equal => Value::Boolean(lf == rf),
            Op::NotEqual => Value::Boolean(lf != rf),
            Op::Mod => {
                if rf == 0.0 {
                    panic!("módulo por cero: {} mod 0.0", lf);
                }
                // resto matemático no negativo
                Value::Real(lf.rem_euclid(rf))
            }
        },

        (Value::Integer(li), Value::Real(rf)) => apply_op(Value::Real(li as f64), op, Value::Real(rf)),
        (Value::Real(lf), Value::Integer(ri)) => apply_op(Value::Real(lf), op, Value::Real(ri as f64)),

        (Value::Integer(lf), Value::Byte(ri)) => apply_op(Value::Integer(lf), op, Value::Integer(ri as i64)),
        (Value::Byte(lf), Value::Integer(ri)) => apply_op(Value::Integer(lf as i64), op, Value::Integer(ri)),
        (Value::Byte(lf), Value::Real(ri)) => apply_op(Value::Real(lf as f64), op, Value::Real(ri)),
        (Value::Real(lf), Value::Byte(ri)) => apply_op(Value::Real(lf), op, Value::Real(ri as f64)),

        (Value::Byte(li), Value::Byte(ri)) => match op {
            Op::Add => {
                let res = li as i16 + ri as i16;
                if res > u8::MAX as i16 {
                    panic!("overflow en operación Byte + Byte: {} + {}", li, ri);
                }
                Value::Byte(res as u8)
            }
            Op::Sub => {
                let res = li as i16 - ri as i16;
                if res < 0 {
                    panic!("underflow en operación Byte - Byte: {} - {}", li, ri);
                }
                Value::Byte(res as u8)
            }
            Op::Mul => {
                let res = li as i16 * ri as i16;
                if res > u8::MAX as i16 {
                    panic!("overflow en operación Byte * Byte: {} * {}", li, ri);
                }
                Value::Byte(res as u8)
            }
            Op::Div => {
                if ri == 0 {
                    panic!("división por cero: {} / 0", li);
                }
                Value::Byte(li / ri)
            }
            Op::Idiv => {
                if ri == 0 {
                    panic!("división por cero: {} / 0", li);
                }
                Value::Byte(li / ri)
            }
            Op::Mod => {
                if ri == 0 {
                    panic!("división por cero: {} / 0", li);
                }
                Value::Byte(li % ri)
            }
            Op::Greater => Value::Boolean(li > ri),
            Op::Less => Value::Boolean(li < ri),
            Op::GreaterEq => Value::Boolean(li >= ri),
            Op::LessEq => Value::Boolean(li <= ri),
            Op::Equal => Value::Boolean(li == ri),
            Op::NotEqual => Value::Boolean(li != ri),
        },

        (Value::Str(ls), Value::Str(rs)) => match op {
            Op::Add => Value::Str(ls + &rs),
            Op::Equal => Value::Boolean(ls == rs),
            Op::NotEqual => Value::Boolean(ls != rs),
            Op::Less => Value::Boolean(ls < rs),
            Op::Greater => Value::Boolean(ls > rs),
            Op::LessEq => Value::Boolean(ls <= rs),
            Op::GreaterEq => Value::Boolean(ls >= rs),
            _ => panic!("Operador no soportado para strings"),
        },

        (Value::Boolean(lb), Value::Boolean(rb)) => match op {
            Op::Equal => Value::Boolean(lb == rb),
            Op::NotEqual => Value::Boolean(lb != rb),
            _ => panic!("Operador no soportado para booleanos"),
        },

        // nil usage
        (Value::Nil, Value::Nil) => match op {
            Op::Equal => Value::Boolean(true),
            Op::NotEqual => Value::Boolean(false),
            // <, <=, >, >= no tienen sentido
            Op::Less | Op::LessEq | Op::Greater | Op::GreaterEq => {
                panic!("can't compare `nil` with `nil`")
            }
            _ => panic!("Operador no soportado para nil"),
        },

        (Value::Nil, rv) => match op {
            Op::Equal => Value::Boolean(false),
            Op::NotEqual => Value::Boolean(true),
            // <, <=, >, >= no tienen sentido
            Op::Less | Op::LessEq | Op::Greater | Op::GreaterEq => {
                panic!("can't compare `nil` with `{:?}`", rv)
            }
            _ => panic!("Operador no soportado con nil"),
        },

        (lv, Value::Nil) => match op {
            Op::Equal => Value::Boolean(false),
            Op::NotEqual => Value::Boolean(true),
            // <, <=, >, >= no tienen sentido
            Op::Less | Op::LessEq | Op::Greater | Op::GreaterEq => {
                panic!("can't compare `{:?}` with `nil`", lv)
            }
            _ => panic!("Operador no soportado con nil"),
        },

        _ => panic!("Operación inválida entre tipos"),
    }
}
