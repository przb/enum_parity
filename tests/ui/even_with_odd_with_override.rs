use enum_parity::bit_parity;

#[bit_parity(even, allow_explicit_overrides = true)]
#[repr(u64)]
enum EvenEnum {
    A,
    B = 0x04,
    C,
    D,
}

fn main() {
    assert_eq!(EvenEnum::A as u64, 0);
    assert_eq!(EvenEnum::B as u64, 4);
    assert_eq!(EvenEnum::C as u64, 5);
    assert_eq!(EvenEnum::D as u64, 6);
}
