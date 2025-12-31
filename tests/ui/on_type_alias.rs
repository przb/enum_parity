use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
enum Foo {
    X,
    Y,
}

#[bit_parity(even)]
type MyFoo = Foo;

fn main() {}
