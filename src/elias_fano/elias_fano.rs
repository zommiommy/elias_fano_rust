use super::*;
use alloc::string::String;

#[derive(Clone, Debug, PartialEq)]
pub struct EliasFano<const QUANTUM_LOG2: usize> {
    pub low_bits: CompactArray,
    pub high_bits: SparseIndex<QUANTUM_LOG2>,

    pub universe: usize,
    pub number_of_elements: usize,

    pub last_high_value: usize,
    pub last_value: usize,
    pub last_index: usize,
    pub current_number_of_elements: usize,
}

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    /// Reduces the memory allocated to the minimum needed.
    pub fn shrink_to_fit(&mut self) {
        self.low_bits.shrink_to_fit();
        self.high_bits.shrink_to_fit();
    }

    #[inline]
    pub(crate) fn extract_high_bits(&self, value: usize) -> usize {
        value >> self.low_bits.word_size()
    }

    #[inline]
    pub(crate) fn extract_low_bits(&self, value: usize) -> usize {
        value & self.low_bits.word_mask()
    }

    #[inline]
    pub(crate) fn extract_high_low_bits(&self, value: usize) -> (usize, usize) {
        // The following is an efficient mod operation
        // It is the equivalent of executing:
        //
        // let low = value % low_bit_count;
        //
        // but faster.
        //
        (self.extract_high_bits(value), self.extract_low_bits(value))
    }

    #[inline]
    pub(crate) fn build_low_high_bits(
        &mut self,
        values: impl Iterator<Item = usize>,
    ) -> Result<(), String> {
        values.map(move |value| self.push(value)).collect()
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
    /// * `value`: usize - Value whose rank is to be extracted.
    ///
    /// # Usage example
    ///
    /// Let's see an example. If I have the vector:
    ///
    /// ```rust
    /// # use elias_fano_rust::elias_fano::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::<10>::from_vec(&vector).unwrap();
    ///
    /// assert_eq!(ef.rank(15).unwrap(), 3);
    /// assert_eq!(ef.rank(8).unwrap(), 1);
    /// ```
    ///
    #[inline]
    pub fn rank(&self, value: usize) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
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
        let mut ones = index - high;
        // handle the case where
        while self.high_bits.get(index) && self.low_bits.read(ones) < low {
            ones += 1;
            index += 1;
        }

        if self.high_bits.get(index) && self.low_bits.read(ones) == low {
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
    /// * `value`: usize - Value whose rank is to be extracted.
    ///
    /// # Usage example
    ///
    /// Let's see an example. If I have the vector:
    ///
    /// ```rust
    /// # use elias_fano_rust::elias_fano::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::<10>::from_vec(&vector).unwrap();
    ///
    /// assert_eq!(ef.unchecked_rank(15), 3);
    /// assert_eq!(ef.unchecked_rank(8), 1);
    /// assert_eq!(ef.unchecked_rank(17), 4);
    /// ```
    ///
    #[inline]
    pub fn unchecked_rank(&self, value: usize) -> usize {
        if self.is_empty() {
            return 0;
        }
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
        let mut ones = index - high;
        // handle the case where
        while self.high_bits.get(index) && self.low_bits.read(ones) < low {
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
    /// * index: usize - Index of the value to be extract.
    #[inline]
    pub fn select(&self, index: usize) -> Result<usize, String> {
        match index < self.number_of_elements {
            true => Ok(self.unchecked_select(index)),
            false =>Err(format!(
                "Given index {} is out of bound on a collection with {} elements.",
                index, self.number_of_elements
                ))
        }
    }

    /// Return the value of the chosen index without executing checks.
    ///
    /// # Arguments
    ///
    /// * index: usize - Index of the value to be extract.
    #[inline]
    pub fn unchecked_select(&self, index: usize) -> usize {
        let high_bits = self.high_bits.select1(index) - index;
        let low_bits = self.low_bits.read(index);
        (high_bits << self.low_bits.word_size()) | low_bits
    }

    #[inline]
    pub fn contains(&self, value: usize) -> bool {
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
        let mut ones = index - high;
        // handle the case where
        while self.high_bits.get(index) && self.low_bits.read(ones) < low {
            ones += 1;
            index += 1;
        }

        self.high_bits.get(index) && self.low_bits.read(ones) == low
    }
}
