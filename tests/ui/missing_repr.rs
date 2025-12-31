use enum_parity::bit_parity;

#[bit_parity(even)]
enum Foo {
    A,
    B,
    C,
}

fn main() {}
