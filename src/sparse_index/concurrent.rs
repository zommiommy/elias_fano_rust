use super::SparseIndex;
use crate::constants::*;

use alloc::{vec::Vec, string::String};
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature="par_iter")]
use rayon::prelude::*;

#[derive(Debug)]
pub struct SparseIndexConcurrentBuilder<const QUANTUM_LOG2: usize> {
    data: Vec<AtomicUsize>, 
    expected_number_of_elements: usize,
    /// Make rust happy about having a fixed index size for a given structure
    phantom: core::marker::PhantomData<[(); QUANTUM_LOG2]>,
}

impl<const QUANTUM_LOG2: usize> SparseIndexConcurrentBuilder<QUANTUM_LOG2> {

    /// Thread safe way to set a value in the bitmap
    pub fn set(&self, index: usize) {
        self.data[(index >> WORD_SHIFT) as usize]
            .fetch_or(1 << (index & WORD_BIT_SIZE_MASK), Ordering::SeqCst);
    }

    /// Finish the building and return the built sparse index
    pub fn build(self) -> Result<SparseIndex<QUANTUM_LOG2>, String> {
        let bitmap = unsafe{ core::mem::transmute::<Vec<_>, Vec<usize>>(self.data) };

        #[cfg(feature="par_iter")]
        let actual_number_of_inserted_values = bitmap.par_iter().map(|x| x.count_ones() as usize).sum::<usize>();
        #[cfg(not(feature="par_iter"))]
        let actual_number_of_inserted_values = bitmap.iter().map(|x| x.count_ones() as usize).sum::<usize>();

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
    pub fn new_concurrent(capacity: usize, number_of_elements: usize) -> SparseIndexConcurrentBuilder<QUANTUM_LOG2> {
        let data: Vec<_> = (0..capacity).map(|_| AtomicUsize::new(0)).collect();

        SparseIndexConcurrentBuilder{
            data,
            expected_number_of_elements: number_of_elements,
            phantom: core::marker::PhantomData::default(),
        }
    }
}