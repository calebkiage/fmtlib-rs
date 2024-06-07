use fmtlib_proc_macros::rt_format_args;
use fmtlib::fmt::{Arg, Value};

fn main() {
    // Args
    let a = rt_format_args!("a": "v", 'b': "v", 'c', 10,);
    assert_eq!(a.len(), 4);
    assert!(matches!(&a[2], Arg::Pos(Value::String(val)) if val == "c"));
    assert!(matches!(&a[3], Arg::Pos(Value::Int64(10))));
    assert!(
        matches!(&a[0], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"a\0")} && v == "v")
    );
    assert!(
        matches!(&a[1], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"b\0")} && v == "v")
    );

    let a = rt_format_args!("arg0", "named": "named arg" );
    assert_eq!(a.len(), 2);
}
