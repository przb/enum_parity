use parity_enum::parity_enum;

#[parity_enum(even)]
#[repr(i32)]
pub enum Sample {
    Foo,
    Bar,
    Baz,
}
