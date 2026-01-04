#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    // valid usage
    t.pass("tests/ui/even_ok.rs");
    t.pass("tests/ui/odd_ok.rs");
    t.pass("tests/ui/non_unit_enums.rs");
    t.pass("tests/ui/cfg_attr.rs");
    t.pass("tests/ui/explicit_discriminant.rs");

    // even parity with discriminants
    t.pass("tests/ui/even_ok_explicit.rs");
    t.compile_fail("tests/ui/even_with_odd_no_override.rs");
    t.pass("tests/ui/even_with_odd_with_override_nonmonotonic.rs");
    t.compile_fail("tests/ui/even_with_odd_with_override_nonmonotonic_reassign.rs");
    t.pass("tests/ui/even_with_odd_with_override.rs");

    // odd parity with discriminants
    t.compile_fail("tests/ui/odd_with_even_no_override.rs");

    // using a const for an enum discriminant
    t.compile_fail("tests/ui/const_discrimin.rs");

    // signed reprs
    t.pass("tests/ui/repr_u8.rs");
    t.pass("tests/ui/repr_i8.rs");

    // invalid or unsupported usage
    t.compile_fail("tests/ui/repr_c.rs");
    t.compile_fail("tests/ui/missing_repr.rs");
    t.compile_fail("tests/ui/bad_arg_err.rs");
    t.compile_fail("tests/ui/overflow_err.rs");

    // on other items
    t.compile_fail("tests/ui/on_struct.rs");
    t.compile_fail("tests/ui/on_mod.rs");
    t.compile_fail("tests/ui/on_trait.rs");
    t.compile_fail("tests/ui/on_use.rs");
    t.compile_fail("tests/ui/on_fn.rs");
    t.compile_fail("tests/ui/on_type_alias.rs");
    t.compile_fail("tests/ui/on_const.rs");
    t.compile_fail("tests/ui/on_impl.rs");
}
