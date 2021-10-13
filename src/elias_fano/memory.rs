use super::*;
use crate::sparse_index::SparseIndexMemoryStats;

#[derive(Clone, Debug)]
pub struct EliasFanoMemoryStats {
    pub metadata: usize,
    pub low_bits: usize,
    pub high_bits: SparseIndexMemoryStats,
}

impl EliasFanoMemoryStats {
    pub fn total(&self) -> usize {
        self.metadata 
        + self.low_bits 
        + self.high_bits.total()
    }
}

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    /// Return the memory used by each sub-element in bytes
    pub fn memory_stats(&self) -> EliasFanoMemoryStats {
        use std::mem::size_of;
        EliasFanoMemoryStats {
            metadata: 8 * size_of::<u64>(),
            low_bits: self.low_bits.size(),
            high_bits: self.high_bits.size(),
        }
    }

    /// Return the memory used in bytes
    pub fn size(&self) -> usize {
        self.memory_stats().total()
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
}