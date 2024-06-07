/// Creates a string by interpolating on a format string.
/// The format string uses [fmtlib](https://fmt.dev)'s syntax.
///
/// The main difference with [`std::format!`] is that the format string does
/// not have to be a compile time literal.
///
/// # Examples
/// ```
/// # use fmtlib::fmt::rt_format;
///
/// // A plain string is printed out as is.
/// let result = rt_format!("Hello");
/// assert_eq!(result.unwrap(), "Hello");
/// ```
///
///```
/// # use fmtlib::fmt::rt_format;
///
/// // An interpolated string.
/// let result = rt_format!("{} {}", "hello", "world");
/// assert_eq!(result.unwrap(), "hello world");
/// ```
///
/// ```
/// # use fmtlib::fmt::rt_format;
///
/// // Positional arguments
/// let result = rt_format!("{0} {0} {1}", 12, 20);
/// assert_eq!(result.unwrap(), "12 12 20");
/// ```
///
/// ```
/// # use fmtlib::fmt::rt_format;
///
/// // Named arguments
/// let result = rt_format!("{a} {a} {b}", a: 12, b: 20);
/// assert_eq!(result.unwrap(), "12 12 20");
/// ```
///
/// ```
/// # use fmtlib::fmt::rt_format;
///
/// // Named and positional
/// let result = rt_format!("{0} {0} {b}", 12, b: 20);
/// assert_eq!(result.unwrap(), "12 12 20");
/// ```
///
/// # Performance
///
/// This macro allocates a new string when constructing the format string. The underlying library
/// also allocates the format arguments.
///
/// # Known issues
///
/// The underlying C++ library doesn't allow mixing automatic & manual arg
/// indexing. What this means is that one has to decide whether to use
/// `{}` or `{<index>}` in a template string. Named arguments are considered
/// to be manually indexed.
///
/// For example:
/// ```ignore
/// // The following format strings will not be interpolated.
///
/// // mixing automatic and manual indexing is not supported
/// "{} {0} {1} {}" // To fix it, use manual indexing "{0} {0} {1} {2}"
///
/// // having an automatically indexed placeholder after a named argument is
/// // not supported either.
/// "{} {named} {}" // To fix it, use manual indexing after the
///                 // named argument "{0} {named} {1}"
/// ```
#[macro_export]
macro_rules! rt_format {
    ($msg:literal) => {Ok::<_, $crate::fmt::errors::Error>($msg.to_string())};
    ($msg:expr) => {Ok::<_, $crate::fmt::errors::Error>($msg.to_string())};
    ($msg:expr, $args:expr) => {unsafe {
        use $crate::fmt::IntoArgs;
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        let args = $args.into_args();
        $crate::ffi::fmt::format(msg.as_ptr(), args.as_slice()).map_err(|e| $crate::fmt::errors::Error::FormatFailed(std::format!("{}", e)))
    }};
    ($msg:tt, $($args:tt)+) => {unsafe {
        use $crate::fmt::Arg;
        let msg = std::ffi::CString::new($msg).expect("cannot construct format string. invalid byte source");
        let args = $crate::fmt::rt_format_args!($($args)+);
        $crate::ffi::fmt::format(msg.as_ptr(), args.as_slice()).map_err(|e| $crate::fmt::errors::Error::FormatFailed(std::format!("{}", e)))
    }};
}

pub use rt_format;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let x = rt_format!("test");
        assert_eq!(x.expect("formatting failed"), "test");
        let x = rt_format!("test {0}", "arg0");
        assert_eq!(x.expect("formatting failed"), "test arg0");
        let x = rt_format!("test {a}", a: "arg0");
        assert_eq!(x.expect("formatting failed"), "test arg0");
        let x = rt_format!("test {0} {named}", "arg0", "named": "named arg" );
        assert_eq!(x.expect("formatting failed"), "test arg0 named arg");
        let x = rt_format!("{a} {a} {b}", a: 12, b: 20 );
        assert_eq!(x.expect("formatting failed"), "12 12 20");
        let x = rt_format!("{0} {b} {0} {2}", 12, b: 20, 21 );
        assert_eq!(x.expect("formatting failed"), "12 20 12 21");
    }
}
