use fmtlib_arg::args;

fn main() {
    let a = args!();
    assert_eq!(a.len(), 0)
}
