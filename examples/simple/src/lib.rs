use parity_enum::bit_parity;

#[repr(u64)]
#[bit_parity(even)]
pub enum Sample {
    Foo,
    Bar,
    Baz,
    Quo,
}
