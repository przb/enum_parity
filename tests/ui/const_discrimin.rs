use enum_parity::bit_parity;

const SOME_B: u64 = 0xff;

#[repr(u64)]
#[bit_parity(even)]
enum EvenEnum {
    A,
    B = SOME_B,
    C,
    D,
}

fn main() {}
