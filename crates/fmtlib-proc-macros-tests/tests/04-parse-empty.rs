use fmtlib_proc_macros::rt_format_args;
use fmtlib_rs::fmt::Arg;

fn main() {
    let a = rt_format_args!();
    assert_eq!(a.len(), 0)
}
