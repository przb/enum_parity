use enum_parity::bit_parity;

struct Foo;

#[bit_parity(even)]
#[repr(u8)]
impl Foo {
    fn new() -> Self {
        Foo
    }
}

fn main() {}
