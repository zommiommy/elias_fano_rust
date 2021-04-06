use super::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::ops::Range;

impl EliasFano {
    #[inline]
    pub fn len(&self) -> usize {
        self.current_number_of_elements as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.current_number_of_elements == 0
    }

    /// Return iterator for the values in elias fano using the old way with selects.
    /// This method is only meant for banchmarking.
    #[inline]
    pub fn iter_select(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.current_number_of_elements).map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.high_bits.iter().enumerate().map(move |(index, high_bit_index)|{
            let high_value = high_bit_index - index as u64;
            let low_bits = self.read_lowbits(index as u64);
            (high_value << self.low_bit_count) | low_bits
        })
    }

    #[inline]
    pub fn iter_in_range(&self, range: Range<u64>) -> impl Iterator<Item = u64> + '_ {
        let Range{
            start,
            end
        } = range;
        
        let offset = self.unchecked_rank(start);
        let high_end   = self.unchecked_rank(end).saturating_add(end >> self.low_bit_count);
        let high_start = offset.saturating_add(start >> self.low_bit_count);

        self.high_bits.iter_in_range(high_start..high_end).enumerate()
            .map(move |(index, high_bit_index)| {
                let index = index as u64 + offset;
                let high_value = high_bit_index - index;
                let low_bits = self.read_lowbits(index);
                (high_value << self.low_bit_count) | low_bits
            })
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn iter_uniques(&self) -> impl Iterator<Item = u64> + '_ {
        let mut last_value = 0;
        let mut first = true;
        self.iter().filter_map(move |value| {
            match first || last_value != value {
                true => {
                    first = false;
                    last_value = value;
                    Some(value)
                }
                false => None,
            }
        })
    }

    /// Return a parallel iterator for the values in elias fano.
    #[inline]
    pub fn par_iter_uniques(&self) -> impl ParallelIterator<Item = u64> + '_ {
        (0..self.current_number_of_elements).into_par_iter()
            .filter_map(move |index| {
                if index == 0 {
                    return Some(self.unchecked_select(0));
                }
                let last_value = self.unchecked_select(index - 1);
                let value = self.unchecked_select(index);

                if last_value != value {
                    Some(value)
                } else {
                    None
                }
            })
    }

    /// Return universe of the elias fano data structure.
    #[inline]
    pub fn get_universe(&self) -> u64 {
        self.universe
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn par_iter(&self) -> impl ParallelIterator<Item = u64> + '_ {
        (0..self.current_number_of_elements)
            .into_par_iter()
            .map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn enumerate(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        (0..self.current_number_of_elements).map(move |index| (index, self.unchecked_select(index)))
    }

    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn par_enumerate(&self) -> impl ParallelIterator<Item = (u64, u64)> + '_ {
        (0..self.current_number_of_elements)
            .into_par_iter()
            .map(move |index| (index, self.unchecked_select(index)))
    }
}
