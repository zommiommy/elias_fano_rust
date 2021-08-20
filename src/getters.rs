use super::*;
use rayon::iter::{ParallelIterator, IndexedParallelIterator};
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

    /// Return universe of the elias fano data structure.
    #[inline]
    pub fn get_universe(&self) -> u64 {
        self.universe
    }
}
