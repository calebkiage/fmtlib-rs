use fmtlib_proc_macros::rt_format_args;
use fmtlib_rs::fmt::{Arg, Value};

fn main() {
    // Args
    let a = rt_format_args!("a": "v", 'b': "v", c: "v", 10: "v");
    assert_eq!(a.len(), 4);
    assert!(
        matches!(&a[0], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"a\0")} && v == "v")
    );
    assert!(
        matches!(&a[1], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"b\0")} && v == "v")
    );
    assert!(
        matches!(&a[2], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"c\0")} && v == "v")
    );
    assert!(
        matches!(&a[3], Arg::Named(k, Value::String(v)) if k.as_c_str() == unsafe{std::ffi::CStr::from_bytes_with_nul_unchecked(b"10\0")} && v == "v")
    );
}
