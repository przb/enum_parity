use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
fn foo() {}

fn main() {}
