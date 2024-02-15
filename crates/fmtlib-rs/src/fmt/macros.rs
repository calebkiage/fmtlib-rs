#[macro_export]
macro_rules! format {
    ($msg:literal) => {Ok::<_, $crate::fmt::errors::Error>(::std::format!("{}", $msg))};
    ($msg:expr, $args:expr) => {unsafe {
        use $crate::fmt::Arg;
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        $crate::ffi::fmt::format(msg.as_ptr(), $args.as_mut_slice()).map_err(|e| $crate::fmt::errors::Error::FormatFailed(std::format!("{}", e)))
    }};
    ($msg:tt, $($args:tt)+) => {unsafe {
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        let mut args = ::fmtlib_arg::args!($($args)+);
        $crate::ffi::fmt::format(msg.as_ptr(), args.as_mut_slice()).map_err(|e| $crate::fmt::errors::Error::FormatFailed(std::format!("{}", e)))
    }};
}

pub use format;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let x = format!("test");
        assert_eq!(x.expect("formatting failed"), "test");
        let x = format!("test {} {named}", "arg0", "named": "named arg" );
        assert_eq!(x.expect("formatting failed"), "test arg0 named arg");
    }
}
