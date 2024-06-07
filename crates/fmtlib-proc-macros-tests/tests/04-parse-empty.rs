use fmtlib_proc_macros::rt_format_args;
use fmtlib::fmt::Arg;

fn main() {
    let a = rt_format_args!();
    assert_eq!(a.len(), 0)
}
