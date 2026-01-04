use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u64)]
enum EvenEnum {
    A,
    // skipping 3
    B = 0x05,
    C,
    D,
}

fn main() {
    assert_eq!(EvenEnum::A as u64, 0);
    assert_eq!(EvenEnum::B as u64, 5);
    assert_eq!(EvenEnum::C as u64, 6);
    assert_eq!(EvenEnum::D as u64, 9);
}
