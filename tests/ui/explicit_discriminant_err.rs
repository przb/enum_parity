use parity_enum::bit_parity;

#[bit_parity(even)]
enum BadEnum {
    A = 0,
    B,
}

fn main() {}
