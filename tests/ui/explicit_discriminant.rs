use enum_parity::bit_parity;

#[repr(u8)]
#[bit_parity(even)]
enum BadEnum {
    A = 0,
    B,
}

fn main() {}
