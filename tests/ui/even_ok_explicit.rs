use enum_parity::bit_parity;

#[bit_parity(even)]
#[repr(u64)]
enum OddEnum {
    A,
    // skipping 3
    B = 0x05,
    C,
    D,
}

fn main() {
    assert_eq!(OddEnum::A as u64, 0);
    assert_eq!(OddEnum::B as u64, 5);
    assert_eq!(OddEnum::C as u64, 6);
    assert_eq!(OddEnum::D as u64, 9);
}
