use parity_enum::bit_parity;

#[bit_parity(even)]
pub enum Sample {
    Foo,
    Bar,
    Baz,
    Quo,
}
