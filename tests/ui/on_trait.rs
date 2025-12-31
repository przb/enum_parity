use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u8)]
trait Foo {}

fn main() {}
