#![expect(
    clippy::cast_possible_truncation,
    reason = "all occurrences of casting are 1"
)]

use crate::Parity;

pub trait IntegerParity: Copy + Sized {
    fn first(parity: Parity) -> Self;
    fn checked_increment(self) -> Option<Self>;
    fn has_parity(self, parity: Parity) -> bool;
}

pub struct BitParityIter<T>
where
    T: IntegerParity,
{
    /// the previous value
    ///
    /// `None` if this is the first iteration, `Some` otherwise
    prev_val: Option<T>,

    parity: Parity,
}

impl<T> BitParityIter<T>
where
    T: IntegerParity,
{
    pub(crate) const fn new(parity: Parity) -> Self {
        Self {
            prev_val: None,
            parity,
        }
    }
}

impl<T> Iterator for BitParityIter<T>
where
    T: IntegerParity,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = if let Some(mut val) = self.prev_val {
            val = val.checked_increment()?;
            while !val.has_parity(self.parity) {
                val = val.checked_increment()?;
            }
            val
        } else {
            T::first(self.parity)
        };

        self.prev_val = Some(val);

        Some(val)
    }
}

macro_rules! unsigned_int_par_impl {
    ($($int:ty),* ) => {
    $(impl IntegerParity for $int {
            fn first(parity: Parity) -> Self { match parity { Parity::Even => 0, Parity::Odd => 1} }
            fn checked_increment(self) -> Option<Self> { self.checked_add(1 as _) }
            fn has_parity(self, parity: Parity) -> bool {
                match parity {
                    Parity::Even => self.count_ones().is_multiple_of(2),
                    Parity::Odd => !self.count_ones().is_multiple_of(2),
                }
            }
        })*
    };
    }
macro_rules! signed_int_par_impl {
    ($($int:ty),* ) => {
    $(impl IntegerParity for $int {
            fn first(parity: Parity) -> Self { match parity { Parity::Even => 0, Parity::Odd => 1} }
            fn checked_increment(self) -> Option<Self> {
                self.cast_unsigned().checked_add(1).map(|val| val.cast_signed())
            }
            fn has_parity(self, parity: Parity) -> bool {
                match parity {
                    Parity::Even => self.count_ones().is_multiple_of(2),
                    Parity::Odd => !self.count_ones().is_multiple_of(2),
                }
            }
        })*
    };
}

unsigned_int_par_impl!(u8, u16, u32, u64, u128, usize);
signed_int_par_impl!(i8, i16, i32, i64, i128, isize);
#[cfg(test)]
mod tests {
    use crate::Parity;

    use super::BitParityIter;
    use itertools::Itertools;

    #[test]
    pub fn small_even_parity() {
        let iter = BitParityIter::<usize>::new(Parity::Even);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x00, 0x3, 0x05, 0x06]);
    }

    #[test]
    pub fn small_odd_parity() {
        let iter = BitParityIter::<usize>::new(Parity::Odd);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x01, 0x2, 0x04, 0x07]);
    }

    #[test]
    pub fn overflowing_u8_even_parity() {
        let iter = BitParityIter::<u8>::new(Parity::Even);
        let v = iter.skip(124).collect_vec();

        assert_eq!(v, [0xf9, 0xfa, 0xfc, 0xff]);
    }

    #[test]
    pub fn overflowing_u8_odd_parity() {
        let iter = BitParityIter::<u8>::new(Parity::Odd);
        let v = iter.skip(124).collect_vec();

        assert_eq!(v, [0xf8, 0xfb, 0xfd, 0xfe]);
    }

    // these `i8`s should be the same as `u8`, since we only care about the bits
    #[test]
    pub fn overflowing_i8_even_parity() {
        let iter = BitParityIter::<i8>::new(Parity::Even);
        let v = iter.skip(124).collect_vec();

        assert_eq!(
            v,
            [0xf9_u8 as i8, 0xfa_u8 as i8, 0xfc_u8 as i8, 0xff_u8 as i8]
        );
    }

    #[test]
    pub fn overflowing_i8_odd_parity() {
        let iter = BitParityIter::<i8>::new(Parity::Odd);
        let v = iter.skip(124).collect_vec();

        assert_eq!(
            v,
            [0xf8_u8 as i8, 0xfb_u8 as i8, 0xfd_u8 as i8, 0xfe_u8 as i8]
        );
    }
}
