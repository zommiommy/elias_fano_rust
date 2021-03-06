use super::*;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

impl EliasFano {
    pub fn len(&self) -> usize {
        self.current_number_of_elements as usize
    }

    pub fn is_empty(&self) -> bool {
        self.current_number_of_elements == 0
    }

    /// Return iterator for the values in elias fano.
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.current_number_of_elements).map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    pub fn iter_new(&self) -> impl Iterator<Item = u64> + '_ {
        self.high_bits.iter().enumerate().map(move |(index, high_bit_index)|{
            let high_value = high_bit_index - index as u64;
            let low_bits = self.read_lowbits(index as u64);
            (high_value << self.low_bit_count) | low_bits
        })
        //(0..self.current_number_of_elements).map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    pub fn iter_uniques(&self) -> impl Iterator<Item = u64> + '_ {
        let mut last_value = 0;
        let mut first = true;
        (0..self.current_number_of_elements).filter_map(move |index| {
            let value = self.unchecked_select(index);
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
    pub fn get_universe(&self) -> u64 {
        self.universe
    }

    /// Return iterator for the values in elias fano.
    pub fn par_iter(&self) -> impl ParallelIterator<Item = u64> + '_ {
        (0..self.current_number_of_elements)
            .into_par_iter()
            .map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    pub fn enumerate(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        (0..self.current_number_of_elements).map(move |index| (index, self.unchecked_select(index)))
    }

    /// Return iterator for the values in elias fano.
    pub fn par_enumerate(&self) -> impl ParallelIterator<Item = (u64, u64)> + '_ {
        (0..self.current_number_of_elements)
            .into_par_iter()
            .map(move |index| (index, self.unchecked_select(index)))
    }
}
