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
    G, // 0x09
    H, // 0x0a
}

fn main() {}
