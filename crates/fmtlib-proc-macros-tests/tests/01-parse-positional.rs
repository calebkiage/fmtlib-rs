use fmtlib_proc_macros::rt_format_args;
use fmtlib::fmt::{Arg, Value};

fn string() -> &'static str {
    "fun"
}

fn main() {
    // Args
    let a = rt_format_args!(
        "a",
        'b',
        3,
        String::from("owned"),
        string(),
        true,
        2 + 2,
        12.3
    );
    assert_eq!(a.len(), 8);
    assert!(matches!(&a[0], Arg::Pos(Value::String(val)) if val == "a"));
    assert!(matches!(&a[1], Arg::Pos(Value::String(val)) if val == "b"));
    assert!(matches!(&a[2], Arg::Pos(Value::Int64(3))));
    assert!(matches!(&a[3], Arg::Pos(Value::String(val)) if val == "owned"));
    assert!(matches!(&a[4], Arg::Pos(Value::String(val)) if val == "fun"));
    assert!(matches!(&a[5], Arg::Pos(Value::Bool(true))));
    assert!(matches!(&a[6], Arg::Pos(Value::Int64(4))));
    assert!(matches!(&a[7], Arg::Pos(Value::Float64(val)) if val.eq(&12.3)));
}
