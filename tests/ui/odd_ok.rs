use enum_parity::bit_parity;

#[bit_parity(odd)]
#[repr(u64)]
enum OddEnum {
    A,
    B,
    C,
    D,
}

fn main() {
    assert_eq!(OddEnum::A as u64, 1);
    assert_eq!(OddEnum::B as u64, 2);
    assert_eq!(OddEnum::C as u64, 4);
    assert_eq!(OddEnum::D as u64, 7);
}
