use crate::constants::*;
use super::SparseIndex;
use std::sync::atomic::{AtomicU64, Ordering};
use rayon::prelude::*;

#[derive(Debug)]
pub struct SparseIndexConcurrentBuilder<const QUANTUM_LOG2: usize> {
    data: Vec<AtomicU64>, 
    expected_number_of_elements: u64,
    /// Make rust happy about having a fixed index size for a given structure
    phantom: std::marker::PhantomData<[(); QUANTUM_LOG2]>,
}

impl<const QUANTUM_LOG2: usize> SparseIndexConcurrentBuilder<QUANTUM_LOG2> {

    /// Thread safe way to set a value in the bitmap
    pub fn set(&self, index: u64) {
        self.data[(index >> WORD_SHIFT) as usize]
            .fetch_or(1 << (index & WORD_MASK), Ordering::SeqCst);
    }

    /// Finish the building and return the built sparse index
    pub fn build(self) -> Result<SparseIndex<QUANTUM_LOG2>, String> {
        let bitmap = unsafe{ std::mem::transmute::<Vec<_>, Vec<u64>>(self.data) };

        let actual_number_of_inserted_values = bitmap.par_iter().map(|x| x.count_ones() as u64).sum::<u64>();

        if actual_number_of_inserted_values != self.expected_number_of_elements {
            return Err(format!(concat!(
                "The number of elements given on construction to EliasFano's concurrent builder was {}",
                " but on the high bits there are {} ones, so either you inserted less elements, or",
                " there were duplicated indices!"
            ),
            self.expected_number_of_elements,
            actual_number_of_inserted_values,
            ));
        }

        Ok(SparseIndex::from_vec(bitmap))
    }
}

impl<const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {

    /// Create a new 
    pub fn new_concurrent(capacity: usize, number_of_elements: u64) -> SparseIndexConcurrentBuilder<QUANTUM_LOG2> {
        let data: Vec<_> = (0..capacity).map(|_| AtomicU64::new(0)).collect();

        SparseIndexConcurrentBuilder{
            data,
            expected_number_of_elements: number_of_elements,
            phantom: std::marker::PhantomData::default(),
        }
    }
}