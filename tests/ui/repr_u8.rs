use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
enum Foo {
    Aa,
    Ab,
    Ac,
    Ad,
    Ae,
    Af,
}

fn main() {}
