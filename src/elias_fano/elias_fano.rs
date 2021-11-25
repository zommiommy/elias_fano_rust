use super::*;
use alloc::string::String;

#[derive(Clone, Debug, PartialEq)]
/// Elias-Fano Quasi-Succint Index by Sebastiano Vigna.
///
/// This index can store a sequence of non-decreasing positive integers using
/// space close to the theoretical minimum.
/// More precisely, given a sequence of values of length $n$ with upperbound
/// (universe) $u$ this datastructure will use:
/// $$ 2 e + e \left \lceil \log_2 \frac{u}{n} \right \rceil$$
pub struct EliasFano<const QUANTUM_LOG2: usize> {
    /// The lowbits are sotred contiguously using a fixed-length binary encoding
    pub(crate) low_bits: CompactArray,
    /// The high-bits are encoded using a sparse-index which with O(n) extra space
    /// allows for O(1) rank and select. In practice this O(n) is ~1% of overhead
    pub(crate) high_bits: SparseIndex<QUANTUM_LOG2>,

    /// The maximum value encodable
    pub(crate) universe: usize,
    /// The number of elements in the encoding
    pub(crate) number_of_elements: usize,

    // Builder arguments
    // TODO!: we should split the builder form the actual datastructure
    /// The last encountered high-bits value, this is used to ensure sorting
    pub(crate) last_high_value: usize,
    /// The last value encountered, this is used to ensure sorting
    pub(crate) last_value: usize,
    /// The index of the last value, this is used to ensure sorting
    pub(crate) last_index: usize,
    /// Number of elements currently pushed, this is used to ensure that
    /// the number of elements inserted matches the one expected.
    pub(crate) current_number_of_elements: usize,
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
    /// * index: usize - Index of the value to be extract.
    #[inline]
    pub fn unchecked_select(&self, index: usize) -> usize {
        let high_bits = self.high_bits.select1(index) - index;
        let low_bits = self.low_bits.read(index);
        (high_bits << self.low_bits.word_size()) | low_bits
    }

    #[inline]
    /// Returns whether the given value is present or not
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
