use super::*;
use core::ops::Range;

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    /// Return iterator for the values in elias fano using the old way with selects.
    /// This method is only meant for banchmarking.
    #[inline]
    pub fn iter_select(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.current_number_of_elements).map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.high_bits
            .iter()
            .enumerate()
            .map(move |(index, high_bit_index)| {
                let high_value = high_bit_index - index as usize;
                let low_bits = self.low_bits.read(index as usize);
                (high_value << self.low_bits.word_size()) | low_bits
            })
    }

    #[inline]
    /// Return a new iterator that is optimized to return only the values in the
    /// given range
    pub fn iter_in_range(&self, range: Range<usize>) -> impl Iterator<Item = usize> + '_ {
        let Range { start, end } = range;

        let offset = self.unchecked_rank(start);
        let high_end = self
            .unchecked_rank(end)
            .saturating_add(end >> self.low_bits.word_size());
        let high_start = offset.saturating_add(start >> self.low_bits.word_size());

        self.high_bits
            .iter_in_range(high_start..high_end)
            .enumerate()
            .map(move |(index, high_bit_index)| {
                let index = index as usize + offset;
                let high_value = high_bit_index - index;
                let low_bits = self.low_bits.read(index);
                (high_value << self.low_bits.word_size()) | low_bits
            })
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn iter_uniques(&self) -> impl Iterator<Item = usize> + '_ {
        let mut last_value = 0;
        let mut first = true;
        self.iter()
            .filter_map(move |value| match first || last_value != value {
                true => {
                    first = false;
                    last_value = value;
                    Some(value)
                }
                false => None,
            })
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.current_number_of_elements).map(move |index| (index, self.unchecked_select(index)))
    }
}
