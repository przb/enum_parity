use parity_enum::bit_parity;

#[bit_parity(even)]
#[repr(C)]
enum Foo {
    A,
    B,
    C,
}

fn main() {}
