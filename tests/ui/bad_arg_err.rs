use bit_parity::bit_parity;

#[bit_parity(foo)]
enum BadArg {
    A,
    B,
}

fn main() {}

