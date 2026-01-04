use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u64)]
enum EvenEnum {
    A,
    B = 0x04,
    C,
    D,
}

fn main() {}
