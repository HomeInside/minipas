///
/// a naive date and time implementation
///
use crate::parser::ast::Value;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn date_fn(_args: Vec<Value>) -> Value {
    let (y, m, d, _, _, _) = now_utc();
    Value::Str(format!("{:04}-{:02}-{:02}", y, m, d))
}

pub fn time_fn(_args: Vec<Value>) -> Value {
    let (_, _, _, hh, mm, ss) = now_utc();
    Value::Str(format!("{:02}:{:02}:{:02}", hh, mm, ss))
}

pub fn date_time_fn(_args: Vec<Value>) -> Value {
    let (y, m, d, hh, mm, ss) = now_utc();
    Value::Str(format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, hh, mm, ss))
}

/// Convierte `SystemTime` → (año, mes, día, hora, minuto, segundo) en UTC
fn now_utc() -> (i64, u32, u32, u32, u32, u32) {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("el reloj del sistema está antes del Unix epoch");

    let secs = i64::try_from(duration.as_secs()).expect("el timestamp excede el rango de i64");

    unix_to_ymdhms(secs)
}

/// Convierte segundos desde 1970-01-01 UTC a fecha/hora (gregoriano)
fn unix_to_ymdhms(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    let sec_per_min: i64 = 60;
    let sec_per_hour = 60 * sec_per_min;
    let sec_per_day = 24 * sec_per_hour;

    // se aplica división euclidiana para timestamps negativos.
    let days = secs.div_euclid(sec_per_day);
    let rem_secs = u32::try_from(secs.rem_euclid(sec_per_day)).expect("resto de segundos fuera de rango");

    let sec_per_min: u32 = 60;
    let sec_per_hour = 60 * sec_per_min;

    let hour = rem_secs / sec_per_hour;
    let min = (rem_secs % sec_per_hour) / sec_per_min;
    let sec = rem_secs % sec_per_min;

    // algoritmo calendario de días julianos
    let z = days + 2_440_588; // epoch 1970-01-01 → JD
    let a = z + 32_044;
    let b = (4 * a + 3).div_euclid(146_097);
    let c = a - (146_097 * b).div_euclid(4);

    let d = (4 * c + 3).div_euclid(1461);
    let e = c - (1461 * d).div_euclid(4);
    let m = (5 * e + 2).div_euclid(153);

    let day = u32::try_from(e - (153 * m + 2).div_euclid(5) + 1).expect("día fuera de rango");

    let month = u32::try_from(m + 3 - 12 * m.div_euclid(10)).expect("mes fuera de rango");

    let year = 100 * b + d - 4800 + m.div_euclid(10);

    (year, month, day, hour, min, sec)
}
