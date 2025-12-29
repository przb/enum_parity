use parity_enum::bit_parity;

#[bit_parity(even)]
#[repr(u64)]
pub enum Sample {
    Foo,
    Bar,
    Baz,
    Quo,
}
