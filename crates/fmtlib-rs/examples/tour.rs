use fmtlib_rs::fmt::rt_format;

fn main() {
    let x = rt_format!("hello world").unwrap();
    assert_eq!(&x, "hello world");

    let f = || "hello {}";
    let x = rt_format!(f(), "world").unwrap();
    assert_eq!(&x, "hello world");

    let str_fmt = "hello {}";
    let x = rt_format!(str_fmt, "world").unwrap();
    assert_eq!(&x, "hello world");
    let x = rt_format!("hello {name}", name: "world").unwrap();
    assert_eq!(&x, "hello world");
}
