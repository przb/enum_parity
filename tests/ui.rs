#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/even_ok.rs");
    t.pass("tests/ui/odd_ok.rs");

    t.pass("tests/ui/repr_u8.rs");
    t.pass("tests/ui/repr_i8.rs");

    t.compile_fail("tests/ui/repr_c.rs");
    t.compile_fail("tests/ui/missing_repr.rs");
    t.compile_fail("tests/ui/explicit_discriminant_err.rs");
    t.compile_fail("tests/ui/bad_arg_err.rs");
    // t.compile_fail("tests/ui/overflow_err.rs");
}
