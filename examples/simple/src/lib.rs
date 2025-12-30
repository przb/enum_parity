use parity_enum::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
pub enum Sample {
    Foo = 1,
    Bar,
    Baz,
    Quo,
}
