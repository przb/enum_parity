use enum_parity::bit_parity;

#[bit_parity(even, allow_explicit_overrides = true)]
#[repr(u64)]
enum OddEnum {
    A, // 0x00
    B, // 0x03
    C = 0x0a,
    D, // 0x0c
    E = 0x05,
    F, // 0x06
}

fn main() {
    assert_eq!(OddEnum::A as u64, 0);
    assert_eq!(OddEnum::B as u64, 3);
    assert_eq!(OddEnum::C as u64, 0x0a);
    assert_eq!(OddEnum::D as u64, 0x0c);
    assert_eq!(OddEnum::E as u64, 0x05);
    assert_eq!(OddEnum::F as u64, 0x06);
}
