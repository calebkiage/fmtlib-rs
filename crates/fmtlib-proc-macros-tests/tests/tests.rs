#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse-positional.rs");
    t.pass("tests/02-parse-named.rs");
    t.pass("tests/03-parse-mixed.rs");
    t.pass("tests/04-parse-empty.rs");
    //t.compile_fail("tests/failing.rs");
}
