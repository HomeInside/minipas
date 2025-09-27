use crate::parser::ast::Value;
use crate::runtime::std_lib::conv::*;
use crate::runtime::std_lib::date_time::*;
use crate::runtime::std_lib::in_out::*;
use crate::runtime::std_lib::math::*;
use crate::runtime::std_lib::strings::*;
use crate::runtime::std_lib::sys::*;
use crate::runtime::std_lib::testing::*;

use std::collections::HashMap;
use std::sync::LazyLock;
pub static BUILTINS: LazyLock<HashMap<String, Builtin>> = LazyLock::new(default_builtins);

pub type RuntimeFn = fn(Vec<Value>) -> Value;
pub type RuntimeProc = fn(Vec<Value>) -> ();

#[derive(Clone)]
pub enum Builtin {
    Const(Value),
    Func(RuntimeFn),
    Proc(RuntimeProc),
}

pub fn default_builtins() -> HashMap<String, Builtin> {
    let mut builtin: HashMap<String, Builtin> = HashMap::new();

    // === input/output ===
    builtin.insert("writeln".to_string(), Builtin::Proc(writeln_fn));
    builtin.insert("write".to_string(), Builtin::Proc(write_fn));
    builtin.insert("readln".to_string(), Builtin::Func(readln_fn));
    builtin.insert("format".to_string(), Builtin::Func(format_fn));

    // === Constantes ===
    builtin.insert("PI".to_string(), Builtin::Const(Value::Real(std::f64::consts::PI)));
    //número de Euler
    builtin.insert("E".to_string(), Builtin::Const(Value::Real(std::f64::consts::E)));

    // === Funciones matemáticas básicas ===
    builtin.insert("pow".to_string(), Builtin::Func(pow_fn));
    builtin.insert("abs".to_string(), Builtin::Func(abs_fn));

    // === Funciones trigonométricas (en radianes) ===
    builtin.insert("sin".to_string(), Builtin::Func(sin_fn));
    builtin.insert("cos".to_string(), Builtin::Func(cos_fn));
    builtin.insert("tan".to_string(), Builtin::Func(tan_fn));
    builtin.insert("cot".to_string(), Builtin::Func(cot_fn));
    builtin.insert("asin".to_string(), Builtin::Func(asin_fn));
    builtin.insert("acos".to_string(), Builtin::Func(acos_fn));
    builtin.insert("atan".to_string(), Builtin::Func(atan_fn));

    // === Funciones logarítmicas y exponenciales ===
    builtin.insert("exp".to_string(), Builtin::Func(exp_fn));
    builtin.insert("ln".to_string(), Builtin::Func(ln_fn));
    builtin.insert("log10".to_string(), Builtin::Func(log10_fn));
    builtin.insert("sqrt".to_string(), Builtin::Func(sqrt_fn));

    // === Funciones auxiliares ===
    builtin.insert("floor".to_string(), Builtin::Func(floor_fn));
    builtin.insert("ceil".to_string(), Builtin::Func(ceil_fn));
    builtin.insert("round".to_string(), Builtin::Func(round_fn));
    builtin.insert("trunc".to_string(), Builtin::Func(trunc_fn));
    builtin.insert("fract".to_string(), Builtin::Func(fract_fn));
    builtin.insert("max".to_string(), Builtin::Func(max_fn));
    builtin.insert("min".to_string(), Builtin::Func(min_fn));
    builtin.insert("sign".to_string(), Builtin::Func(sign_fn));

    // === sys ===
    builtin.insert("random".to_string(), Builtin::Func(random_fn));
    // alias: delay()
    builtin.insert("sleep".to_string(), Builtin::Proc(sleep_fn));
    builtin.insert("platform".to_string(), Builtin::Func(platform_fn));
    builtin.insert("version".to_string(), Builtin::Func(version_fn));
    builtin.insert("exit".to_string(), Builtin::Proc(exit_fn));
    //alias: clear(), clearscreen()
    builtin.insert("clrscr".to_string(), Builtin::Proc(clear_screen_fn));

    // === date_time ===
    builtin.insert("date".to_string(), Builtin::Func(date_fn));
    builtin.insert("time".to_string(), Builtin::Func(time_fn));
    builtin.insert("date_time".to_string(), Builtin::Func(date_time_fn));

    // === strings ===
    builtin.insert("len".to_string(), Builtin::Func(len_fn));
    // alias: len()
    builtin.insert("length".to_string(), Builtin::Func(len_fn));
    builtin.insert("upper".to_string(), Builtin::Func(upper_fn));
    builtin.insert("lower".to_string(), Builtin::Func(lower_fn));
    builtin.insert("trim".to_string(), Builtin::Func(trim_fn));
    builtin.insert("concat".to_string(), Builtin::Func(concat_fn));

    // === conversiones de tipos ===
    // alias: int(), to_int()
    builtin.insert("to_int".to_string(), Builtin::Func(to_int_fn));
    // alias: real(), to_real()
    builtin.insert("to_real".to_string(), Builtin::Func(to_real_fn));
    // alias: str(), to_str()
    builtin.insert("to_str".to_string(), Builtin::Func(to_str_fn));

    // === testing ===
    builtin.insert("assert".to_string(), Builtin::Proc(assert_fn));
    builtin.insert("assert_equals".to_string(), Builtin::Proc(assert_eq_fn));
    builtin.insert("panic".to_string(), Builtin::Proc(panic_fn));

    builtin
}
