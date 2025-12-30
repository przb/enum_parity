use parity_enum::bit_parity;

#[bit_parity(foo)]
enum BadArg {
    A,
    B,
}

fn main() {}
