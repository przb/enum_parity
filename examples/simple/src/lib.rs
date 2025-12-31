use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
pub enum EvenParitySample {
    Foo,
    Bar,
    Baz,
    Quo,
}

#[bit_parity(odd)]
#[repr(u8)]
pub enum OddParitySample {
    Lorem,
    Ipsum,
    Dolor,
    Sit,
}
