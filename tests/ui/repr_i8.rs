use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(i8)]
enum Foo {
    Aa,
    Ab,
    Ac,
    Ad,
    Ae,
    Af,
}

fn main() {}
