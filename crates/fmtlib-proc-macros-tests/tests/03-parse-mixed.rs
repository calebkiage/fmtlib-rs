use fmtlib_arg::args;
use fmtlib_rs::fmt::{Arg, Value};

fn main() {
    // Args
    let a = args!("a": "v", 'b': "v", 'c', 10,);
    assert_eq!(a.len(), 4);
    assert!(matches!(&a[0], Arg::Pos(Value::String(val)) if val == "c"));
    assert!(matches!(&a[1], Arg::Pos(Value::Int64(10))));
    assert!(
        matches!(&a[2], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"a\0")} && v == "v")
    );
    assert!(
        matches!(&a[3], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"b\0")} && v == "v")
    );

    let a = args!("arg0", "named": "named arg" );
    assert_eq!(a.len(), 2);
}
