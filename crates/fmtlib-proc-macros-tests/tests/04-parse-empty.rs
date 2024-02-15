use fmtlib_proc_macros::format_args;
use fmtlib_rs::fmt::Arg;

fn main() {
    let a = format_args!();
    assert_eq!(a.len(), 0)
}
