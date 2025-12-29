pub(crate) struct BitParityIter {
    /// the previous value
    ///
    /// `None` if this is the first iteration, `Some` otherwise
    prev_val: Option<u64>,

    /// `true` if there should be an even number of ones, `false` otherwise
    is_even_parity: bool,
}

impl BitParityIter {
    pub(crate) fn new(is_even: bool) -> Self {
        Self {
            prev_val: None,
            is_even_parity: is_even,
        }
    }
}

impl Iterator for BitParityIter {
    type Item = u64;

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
}

#[cfg(test)]
mod tests {
    use super::BitParityIter;
    use itertools::Itertools;

    #[test]
    pub fn small_even_parity() {
        let iter = BitParityIter::new(true);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x00, 0x3, 0x05, 0x06])
    }

    #[test]
    pub fn small_odd_parity() {
        let iter = BitParityIter::new(false);
        let v = iter.take(4).collect_vec();

        assert_eq!(v, [0x01, 0x2, 0x04, 0x07])
    }
}
