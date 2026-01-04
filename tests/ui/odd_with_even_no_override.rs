use enum_parity::bit_parity;

#[bit_parity(odd)]
#[repr(u64)]
enum OddEnum {
    A,
    B = 0x03,
    C,
    D,
}

fn main() {}
