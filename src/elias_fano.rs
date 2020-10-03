use super::*;
use fid::{BitVector, FID};


#[derive(Clone, Debug, PartialEq)]
pub struct EliasFano {
    pub(crate) universe: u64,
    pub(crate) number_of_elements: u64,
    pub(crate) low_bit_count: u64,
    pub(crate) low_bit_mask: u64,
    pub(crate) low_bits: Vec<u64>,
    pub(crate) high_bits: BitVector,
    pub(crate) last_high_value: u64,
    pub(crate) last_value: u64,
    pub(crate) last_index: u64,
    pub(crate) current_number_of_elements: u64,
}

impl EliasFano {

    pub(crate) fn extract_high_bits(&self, value: u64) -> u64 {
        value >> self.low_bit_count
    }

    pub(crate) fn extract_low_bits(&self, value: u64) -> u64 {
        value & self.low_bit_mask
    }

    pub(crate) fn extract_high_low_bits(&self, value: u64) -> (u64, u64) {
        // The following is an efficient mod operation
        // It is the equivalent of executing:
        //
        // let low = value % low_bit_count;
        //
        // but faster.
        //
        (self.extract_high_bits(value), self.extract_low_bits(value))
    }


    pub(crate) fn build_low_high_bits(&mut self, values: impl Iterator<Item = u64>) -> Result<(), String> {
        values.map(move |value| self.push(value)).collect()
    }

    pub(crate) fn read_lowbits(&self, index: u64) -> u64 {
        safe_read(&self.low_bits, index, self.low_bit_count)
    }


    /// Return the number of elements <= to the given value.
    /// If the element is in the set, this is equivalent to the
    /// index of the first instance of the given value.
    ///
    /// This means that if in the vector there are multiple equal values,
    /// the index returned will always be the one of the first.
    ///
    /// # Arguments
    ///
    /// * `value`: u64 - Value whose rank is to be extracted.
    ///
    /// # Usage example
    ///
    /// Let's see an example. If I have the vector:
    ///
    /// ```rust
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_vec(&vector).unwrap();
    ///
    /// assert_eq!(ef.rank(15).unwrap(), 3);
    /// assert_eq!(ef.rank(8).unwrap(), 1);
    /// ```
    ///
    pub fn rank(&self, value: u64) -> Option<u64> {
        if value > self.last_value {
            return None;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1,
        };
        // get the first guess
        let mut ones = self.high_bits.rank1(index);
        // handle the case where
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }

        if self.high_bits.get(index) && self.read_lowbits(ones) == low {
            Some(ones)
        } else {
            None
        }
    }

    /// Return the number of elements <= to the given value.
    /// If the element is in the set, this is equivalent to the
    /// index of the first instance of the given value.
    ///
    /// This means that if in the vector there are multiple equal values,
    /// the index returned will always be the one of the first.
    ///
    /// # Arguments
    ///
    /// * `value`: u64 - Value whose rank is to be extracted.
    ///
    /// # Usage example
    ///
    /// Let's see an example. If I have the vector:
    ///
    /// ```rust
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_vec(&vector).unwrap();
    ///
    /// assert_eq!(ef.unchecked_rank(15), 3);
    /// assert_eq!(ef.unchecked_rank(8), 1);
    /// assert_eq!(ef.unchecked_rank(17), 4);
    /// ```
    ///
    pub fn unchecked_rank(&self, value: u64) -> u64 {
        if value > self.last_value {
            return self.current_number_of_elements;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1,
        };
        // get the first guess
        let mut ones = self.high_bits.rank1(index);
        // handle the case where
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }

        ones
    }

    /// Return the value of the chosen index.
    ///
    /// This version of the select does a check to verify that the given index
    /// is not greater than the allowed amount. To avoid this check in conditions
    /// where the performance is key, consider using the method `unchecked_select`
    /// which raises a panic when the conditions are not met.
    ///
    /// # Arguments
    ///
    /// * index: u64 - Index of the value to be extract.
    pub fn select(&self, index: u64) -> Result<u64, String> {
        match index < self.number_of_elements {
            true => Ok(self.unchecked_select(index)),
            false => Err(format!(
                "Given index {} is out of bound on a collection with {} elements.",
                index, self.number_of_elements
            )),
        }
    }

    /// Return the value of the chosen index without executing checks.
    ///
    /// # Arguments
    ///
    /// * index: u64 - Index of the value to be extract.
    pub fn unchecked_select(&self, index: u64) -> u64 {
        let bit_index = self.high_bits.select1(index);
        let high_bits = self.high_bits.rank0(bit_index);
        let low_bits = self.read_lowbits(index);
        (high_bits << self.low_bit_count) | low_bits
    }

    pub fn contains(&self, value: u64) -> bool {
        if value > self.last_value {
            return false;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1,
        };
        // get the first guess
        let mut ones = self.high_bits.rank1(index);
        // handle the case where
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }

        self.high_bits.get(index) && self.read_lowbits(ones) == low
    }
}
