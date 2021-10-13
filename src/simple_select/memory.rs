use super::*;

#[derive(Clone, Debug)]
/// Memory usage in bytes by the variuos fields of Simple Select
pub struct SimpleSelectMemoryStats {
    pub high_bits: usize,
    pub metadata: usize,
    pub high_bits_index_zeros: usize,
    pub high_bits_index_ones: usize,
}

impl SimpleSelectMemoryStats {
    pub fn total(&self) -> usize {
        self.high_bits
        + self.high_bits_index_ones
        + self.high_bits_index_ones
        + self.metadata
    }
}

impl SimpleSelect {
    /// Return the memory used in bytes
    pub fn size(&self) -> SimpleSelectMemoryStats {
        use std::mem::size_of;
        SimpleSelectMemoryStats {
            metadata: 3 * size_of::<u64>(),
            high_bits:  (self.high_bits.capacity() * size_of::<u64>()) + size_of::<Vec<u64>>(),
            high_bits_index_zeros:  (self.high_bits_index_zeros.capacity() * size_of::<u64>()) + size_of::<Vec<u64>>(),
            high_bits_index_ones:  (self.high_bits_index_ones.capacity() * size_of::<u64>()) + size_of::<Vec<u64>>(),

        }
    }

    /// Reduces the memory allocated to the minimum needed.
    pub fn shrink_to_fit(&mut self) {
        self.high_bits.shrink_to_fit();
        self.high_bits_index_zeros.shrink_to_fit();
        self.high_bits_index_ones.shrink_to_fit();
    }
}
