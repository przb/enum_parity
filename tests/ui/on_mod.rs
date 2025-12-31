use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
mod foo {
    struct Bar;
}

fn main() {}
