use parity_enum::bit_parity;

#[bit_parity(even)]
enum Foo {
    A,
    B,
    C,
}

fn main() {}
