pub(crate) struct BitParityIter<T> {
    /// the previous value
    ///
    /// `None` if this is the first iteration, `Some` otherwise
    prev_val: Option<T>,

    /// `true` if there should be an even number of ones, `false` otherwise
    is_even_parity: bool,
}

impl<T> BitParityIter<T> {
    pub(crate) fn new(is_even: bool) -> Self {
        Self {
            prev_val: None,
            is_even_parity: is_even,
        }
    }
}

macro_rules! bit_par_iter_impl {
    ($($int:ty),*) => {
        $(impl Iterator for BitParityIter<$int> {
            type Item = $int;

            fn next(&mut self) -> Option<Self::Item> {
                let val = if let Some(mut val) = self.prev_val {
                    val = val.checked_add(1)?;
                    while val.count_ones().is_multiple_of(2) != self.is_even_parity {
                        val = val.checked_add(1)?;
                    }
                    val
                } else {
                    if self.is_even_parity { 0 } else { 1 }
                };

                self.prev_val = Some(val);

                Some(val)
            }
        })*
    };
}

bit_par_iter_impl!(
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
);

#[cfg(test)]
mod tests {
    use super::BitParityIter;
    use itertools::Itertools;

    #[test]
    pub fn small_even_parity() {
        let iter = BitParityIter::<usize>::new(true);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x00, 0x3, 0x05, 0x06])
    }

    #[test]
    pub fn small_odd_parity() {
        let iter = BitParityIter::<usize>::new(false);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x01, 0x2, 0x04, 0x07])
    }
}
