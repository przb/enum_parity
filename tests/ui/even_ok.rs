use bit_parity::bit_parity;

#[bit_parity(even)]
enum EvenEnum {
A,
B,
C,
D,
}

fn main() {
assert_eq!(EvenEnum::A as u64, 0);
assert_eq!(EvenEnum::B as u64, 3);
assert_eq!(EvenEnum::C as u64, 5);
assert_eq!(EvenEnum::D as u64, 6);
}

