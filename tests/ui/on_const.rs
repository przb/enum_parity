use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
const FOO: u8 = 67;

fn main() {}
