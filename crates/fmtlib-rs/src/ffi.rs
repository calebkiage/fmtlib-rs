type Arg<'s> = crate::fmt::Arg<'s>;
type Value<'s> = crate::fmt::Value<'s>;

#[cxx::bridge]
pub mod fmt {

    #[namespace = "shimrs"]
    extern "Rust" {
        type Value<'s>;

        fn is_bool(self: &Value) -> bool;
        fn is_float64(self: &Value) -> bool;
        fn is_int64(self: &Value) -> bool;
        fn is_string(self: &Value) -> bool;

        fn get_bool(self: &Value) -> Result<bool>;
        fn get_int64(self: &Value) -> Result<i64>;
        fn get_float64(self: &Value) -> Result<f64>;
        unsafe fn get_string<'s>(self: &'s Value<'s>) -> Result<&'s str>;
    }

    #[namespace = "shimrs"]
    extern "Rust" {
        type Arg<'s>;

        fn is_named(self: &Arg) -> bool;
        fn is_positional(self: &Arg) -> bool;

        unsafe fn get_name_ptr<'s>(self: &'s Arg<'s>) -> Result<*const c_char>;
        unsafe fn get_value<'s>(self: &'s Arg<'s>) -> &'s Value<'s>;
    }

    #[namespace = "shimcpp"]
    unsafe extern "C++" {
        include!("fmtlib-rs/include/shim.h");

        /// Format a string using [fmtlib](https://fmt.dev)
        /// Due to [issue #3817](https://github.com/fmtlib/fmt/issues/3817)
        /// with the library, named arguments should come last
        #[allow(dead_code)]
        pub(crate) unsafe fn format(fmt: *const c_char, args: &mut [Arg]) -> Result<String>;
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn test_name() {
        let mut args =
            fmtlib_arg::args!(String::from("string"), "static str", 12, "named": "named arg");
        let fmt = CStr::from_bytes_with_nul(b"test '{}' '{}' '{}' '{named}'\0").unwrap();
        match unsafe { fmt::format(fmt.as_ptr(), args.as_mut_slice()) } {
            Ok(ref v) => {
                assert_eq!(v, "test 'string' 'static str' '12' 'named arg'");
            }
            Err(e) => panic!("error from cpp: {e}"),
        }
        let mut args = fmtlib_arg::args!(1, 3, b: 2);
        let fmt = CStr::from_bytes_with_nul(b"test '{}' '{b}' '{}'\0").unwrap();
        match unsafe { fmt::format(fmt.as_ptr(), args.as_mut_slice()) } {
            Ok(ref v) => {
                assert_eq!(v, "test '1' '2' '3'");
            }
            Err(e) => panic!("error from cpp: {e}"),
        }
    }
}
