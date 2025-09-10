///
/// a naive date and time implementation
///
use crate::Value;

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
fn now_utc() -> (i32, u32, u32, u32, u32, u32) {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    unix_to_ymdhms(secs as i64)
}

/// Convierte segundos desde 1970-01-01 UTC a fecha/hora (gregoriano)
fn unix_to_ymdhms(secs: i64) -> (i32, u32, u32, u32, u32, u32) {
    let sec_per_min = 60;
    let sec_per_hour = 60 * sec_per_min;
    let sec_per_day = 24 * sec_per_hour;

    let days = secs / sec_per_day;
    let mut rem_secs = secs % sec_per_day;
    if rem_secs < 0 {
        rem_secs += sec_per_day;
    }

    let hour = (rem_secs / sec_per_hour) as u32;
    let min = ((rem_secs % sec_per_hour) / sec_per_min) as u32;
    let sec = (rem_secs % sec_per_min) as u32;

    // algoritmo calendario de días julianos
    let z = days + 2440588; // epoch 1970-01-01 → JD
    let a = z + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;

    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;

    let day = (e - (153 * m + 2) / 5 + 1) as u32;
    let month = (m + 3 - 12 * (m / 10)) as u32;
    let year = (100 * b + d - 4800 + (m / 10)) as i32;

    (year, month, day, hour, min, sec)
}
