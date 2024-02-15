use fmtlib_proc_macros::args;
use fmtlib_rs::fmt::Arg;

fn main() {
    let a = args!();
    assert_eq!(a.len(), 0)
}
