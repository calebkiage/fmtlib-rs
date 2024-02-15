/// Creates a string by interpolating on a format string.
/// The format string uses [fmtlib](https://fmt.dev)'s syntax.
///
/// The main difference with [`std::format!`] is that the format string does
/// not have to be a compile time literal.
///
/// # Examples
/// ```
/// use fmtlib_rs::fmt::format;
///
/// // A plain string is printed out as is.
/// let result = format!("Hello");
/// assert_eq!(result.unwrap(), "Hello");
///
/// // An interpolated string.
/// let result = format!("{} {}", "hello", "world");
/// assert_eq!(result.unwrap(), "hello world");
///
/// // Positional arguments
/// let result = format!("{0} {0} {1}", 12, 20);
/// assert_eq!(result.unwrap(), "12 12 20");
///
/// // Named arguments
/// let result = format!("{a} {a} {b}", a: 12, b: 20);
/// assert_eq!(result.unwrap(), "12 12 20");
/// ```
#[macro_export]
macro_rules! format {
    ($msg:literal) => {Ok::<_, $crate::fmt::errors::Error>(::std::format!("{}", $msg))};
    ($msg:expr, $args:expr) => {unsafe {
        use $crate::fmt::Arg;
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        $crate::ffi::fmt::format(msg.as_ptr(), $args.as_mut_slice()).map_err(|e| $crate::fmt::errors::Error::FormatFailed(std::format!("{}", e)))
    }};
    ($msg:tt, $($args:tt)+) => {unsafe {
        use $crate::fmt::Arg;
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        let mut args = ::fmtlib_proc_macros::args!($($args)+);
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
        let x = format!("{a} {a} {b}", a: 12, b: 20 );
        assert_eq!(x.expect("formatting failed"), "12 12 20");
    }
}
