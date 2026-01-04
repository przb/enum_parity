use enum_parity::bit_parity;

#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foou8 {
    A,
    B,
}
#[repr(u16)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foou16 {
    A,
    B,
}
#[repr(u32)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foou32 {
    A,
    B,
}
#[repr(u64)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foou64 {
    A,
    B,
}
#[repr(u128)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foou128 {
    A,
    B,
}
#[repr(usize)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Foousize {
    A,
    B,
}

#[repr(i8)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooi8 {
    A,
    B,
}
#[repr(i16)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooi16 {
    A,
    B,
}
#[repr(i32)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooi32 {
    A,
    B,
}
#[repr(i64)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooi64 {
    A,
    B,
}
#[repr(i128)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooi128 {
    A,
    B,
}
#[repr(isize)]
#[derive(Eq, PartialEq, Debug)]
#[bit_parity(even)]
enum Fooisize {
    A,
    B,
}

fn main() {
    assert_eq!(Foou8::A as u8, 0x00_u8);
    assert_eq!(Foou8::B as u8, 0x03_u8);

    assert_eq!(Foou16::A as u16, 0x00_u16);
    assert_eq!(Foou16::B as u16, 0x03_u16);

    assert_eq!(Foou32::A as u32, 0x00_u32);
    assert_eq!(Foou32::B as u32, 0x03_u32);

    assert_eq!(Foou64::A as u64, 0x00_u64);
    assert_eq!(Foou64::B as u64, 0x03_u64);

    assert_eq!(Foou128::A as u128, 0x00_u128);
    assert_eq!(Foou128::B as u128, 0x03_u128);

    assert_eq!(Foousize::A as usize, 0x00_usize);
    assert_eq!(Foousize::B as usize, 0x03_usize);

    assert_eq!(Fooi8::A as i8, 0x00_i8);
    assert_eq!(Fooi8::B as i8, 0x03_i8);

    assert_eq!(Fooi16::A as i16, 0x00_i16);
    assert_eq!(Fooi16::B as i16, 0x03_i16);

    assert_eq!(Fooi32::A as i32, 0x00_i32);
    assert_eq!(Fooi32::B as i32, 0x03_i32);

    assert_eq!(Fooi64::A as i64, 0x00_i64);
    assert_eq!(Fooi64::B as i64, 0x03_i64);

    assert_eq!(Fooi128::A as i128, 0x00_i128);
    assert_eq!(Fooi128::B as i128, 0x03_i128);

    assert_eq!(Fooisize::A as isize, 0x00_isize);
    assert_eq!(Fooisize::B as isize, 0x03_isize);
}
