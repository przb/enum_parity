use enum_parity::bit_parity;

#[bit_parity(even, allow_explicit_overrides = true)]
#[repr(u64)]
enum OddEnum {
    A,
    B = 0x04,
    C,
    D,
}

fn main() {
    assert_eq!(OddEnum::A as u64, 0);
    assert_eq!(OddEnum::B as u64, 4);
    assert_eq!(OddEnum::C as u64, 5);
    assert_eq!(OddEnum::D as u64, 6);
}
