use enum_parity::bit_parity;

#[repr(u64)]
#[bit_parity(even)]
enum EvenEnum {
    A,
    B(u32),
    D(u8, Vec<[i128; 32]>),
    C { x: i32, y: String },
}

fn main() {}
