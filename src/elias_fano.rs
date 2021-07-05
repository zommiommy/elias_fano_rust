use super::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct EliasFano {
    pub low_bits: Vec<u64>,
    pub high_bits: SimpleSelect,
    pub universe: u64,
    pub number_of_elements: u64,
    pub low_bit_count: u64,
    pub low_bit_mask: u64,
    pub last_high_value: u64,
    pub last_value: u64,
    pub last_index: u64,
    pub current_number_of_elements: u64,
}

#[derive(Clone, Debug)]
pub struct EliasFanoMemoryStats {
    pub metadata: usize,
    pub low_bits: usize,
    pub high_bits: SimpleSelectMemoryStats,
}

impl EliasFano {
    /// Return the memory used in bytes
    /// This approximate the metadata as 3 u64 for each vector.
    /// with values specified for each substructure
    pub fn memory_stats(&self) -> EliasFanoMemoryStats {
        use std::mem::size_of;
        EliasFanoMemoryStats {
            metadata: 8 * size_of::<u64>(),
            low_bits: (3 + self.low_bits.capacity()) * size_of::<u64>(),
            high_bits: self.high_bits.size(),
        }
    }

    /// Return the memory used in bytes
    /// This approximate the metadata as 3 u64 for each vector.
    pub fn size(&self) -> usize {
        let vals = self.memory_stats();
        vals.metadata 
        + vals.low_bits 
        + vals.high_bits.high_bits 
        + vals.high_bits.high_bits_index_ones
        + vals.high_bits.high_bits_index_ones
        + vals.high_bits.metadata
    }

    /// Return how much memory is spent for the indices
    /// needed for constant time rank and select in ratio with
    /// the high and low bits vectors.
    pub fn overhead(&self) -> usize {
        let vals = self.memory_stats();
    
        vals.high_bits.high_bits_index_zeros 
        + vals.high_bits.high_bits_index_ones 
        + vals.high_bits.metadata 
        + vals.metadata  
    }

    /// Return how much memory is spent for the indices
    /// needed for constant time rank and select in ratio with
    /// the high and low bits vectors.
    pub fn overhead_ratio(&self) -> f64 {
        let vals = self.memory_stats();
        (
            vals.high_bits.high_bits_index_zeros 
            + vals.high_bits.high_bits_index_ones 
            + vals.high_bits.metadata 
            + vals.metadata  
        ) as f64 / (
            vals.high_bits.high_bits 
            + vals.low_bits 
        ) as f64
    }

    /// Return how much memory is spent for the indices
    /// needed for constant time rank and select in ratio with
    /// the high bits vector.
    pub fn overhead_high_bits_ratio(&self) -> f64 {
        let vals = self.memory_stats();
        (
            vals.high_bits.high_bits_index_zeros 
            + vals.high_bits.high_bits_index_ones 
            + vals.high_bits.metadata 
            + vals.metadata  
        ) as f64 / vals.high_bits.high_bits as f64
    }

    /// Reduces the memory allocated to the minimum needed.
    pub fn shrink_to_fit(&mut self) {
        self.low_bits.shrink_to_fit();
        self.high_bits.shrink_to_fit();
    }

    #[inline]
    pub(crate) fn extract_high_bits(&self, value: u64) -> u64 {
        value >> self.low_bit_count
    }

    #[inline]
    pub(crate) fn extract_low_bits(&self, value: u64) -> u64 {
        value & self.low_bit_mask
    }

    #[inline]
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

    #[inline]
    pub(crate) fn build_low_high_bits(
        &mut self,
        values: impl Iterator<Item = u64>,
    ) -> Result<(), String> {
        values.map(move |value| self.push(value)).collect()
    }

    #[inline]
    pub(crate) fn read_lowbits(&self, index: u64) -> u64 {
        #[cfg(not(feature = "unsafe"))]
        return safe_read(&self.low_bits, index, self.low_bit_count);
        #[cfg(feature = "unsafe")]
        return unsafe_read(&self.low_bits, index, self.low_bit_count);
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
    #[inline]
    pub fn rank(&self, value: u64) -> Option<u64> {
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
    #[inline]
    pub fn unchecked_rank(&self, value: u64) -> u64 {
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
    #[inline]
    pub fn select(&self, index: u64) -> Result<u64, String> {
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
    /// * index: u64 - Index of the value to be extract.
    #[inline]
    pub fn unchecked_select(&self, index: u64) -> u64 {
        let high_bits = self.high_bits.select1(index) - index;
        let low_bits = self.read_lowbits(index);
        (high_bits << self.low_bit_count) | low_bits
    }

    #[inline]
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
        let mut ones = index - high;
        // handle the case where
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }

        self.high_bits.get(index) && self.read_lowbits(ones) == low
    }
}
