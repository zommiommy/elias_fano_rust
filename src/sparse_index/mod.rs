//! This is a sparse index over a bitmap.
//! 
//! In this index we use two vectors to save the position of every 
//! `2**QUANTUM_LOG2` zeros and ones to be able to execute ranks and selects
//! in constant time in average.
use super::utils::*;
use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::constants::*;
use crate::traits::MemoryFootprint;

mod iter;
pub use iter::*;
mod iter_double_ended;
pub use iter_double_ended::*;
mod getters;
mod concurrent;
pub use concurrent::*;

#[cfg(feature="par_iter")]
mod par_iter;

#[derive(Clone, Debug)]
/// This structure is efficient for **DENSE** bitvectors
/// Sparse Index which stores the position of every q-th 1 and 0
pub struct SparseIndex<const QUANTUM_LOG2: usize> {
    pub(crate) high_bits: Vec<usize>,
    pub(crate) high_bits_index_zeros: Vec<usize>,
    pub(crate) high_bits_index_ones: Vec<usize>,
    pub(crate) number_of_ones: usize,
    pub(crate) number_of_zeros: usize,
    pub(crate) len: usize,

    /// Make rust happy about having a fixed index size for a given structure
    phantom: core::marker::PhantomData<[(); QUANTUM_LOG2]>,
}

impl<const QUANTUM_LOG2: usize> MemoryFootprint for SparseIndex<QUANTUM_LOG2> {
    fn total_size(&self) -> usize {
        self.high_bits.total_size() + 
        self.high_bits_index_ones.total_size() +
        self.high_bits_index_zeros.total_size() +
        (core::mem::size_of::<usize>() * 3)
    }
}

impl<const QUANTUM_LOG2: usize> PartialEq for SparseIndex<QUANTUM_LOG2> {
    fn eq(&self, other: &SparseIndex<QUANTUM_LOG2>) -> bool {
        // if needed this can be sped up by comparing the metadata before the vec
        self.high_bits == other.high_bits
    }
}

/// # Constructors
impl<const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {
    /// Allocate an empty high-bits structure
    pub fn new() -> SparseIndex<QUANTUM_LOG2> {
        SparseIndex{
            high_bits: Vec::new(),
            high_bits_index_zeros: Vec::new(),
            high_bits_index_ones: Vec::new(),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
            phantom: core::marker::PhantomData::default(),
        }
    }

    /// Allocate the high-bits with the right size for optimal speed
    pub fn with_capacity(capacity: usize) -> SparseIndex<QUANTUM_LOG2> {
        SparseIndex{
            high_bits: Vec::with_capacity(capacity >> WORD_SHIFT),
            high_bits_index_zeros: Vec::with_capacity(capacity >> QUANTUM_LOG2),
            high_bits_index_ones: Vec::with_capacity(capacity >> QUANTUM_LOG2),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
            phantom: core::marker::PhantomData::default(),
        }
    }

    /// Reduce the memory allocations to the minimum needed
    pub fn shrink_to_fit(&mut self){
        self.high_bits.shrink_to_fit();
        self.high_bits_index_zeros.shrink_to_fit();
        self.high_bits_index_ones.shrink_to_fit();
    }

    /// Add the given bit to the end of the high-bits
    pub fn push(&mut self, value: bool) {
        if value {
            if self.number_of_ones & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                self.high_bits_index_ones.push(self.len);
            }
            self.number_of_ones += 1;
        } else {
            if self.number_of_zeros & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                self.high_bits_index_zeros.push(self.len);
            }
            self.number_of_zeros += 1;
        }

        if self.len & WORD_BIT_SIZE_MASK == 0{
            self.high_bits.push(0);
        }

        if value {
            let last_idx = self.high_bits.len() - 1;
            let mut code = self.high_bits[last_idx];
            code |= 1 << (self.len & WORD_BIT_SIZE_MASK);
            self.high_bits[last_idx] = code;
        }

        self.len += 1;
    }

    /// Take the given bit-vector and build the indices on it.
    pub fn from_vec(bitvector: Vec<usize>) -> SparseIndex<QUANTUM_LOG2> {

        let bitvector = Arc::new(bitvector);

        // The following two steps are independant so we could parallelize them
        // using two separate threads
        // moreover, if we know in advance the number of ones and zeros in the bitvector
        // we can use 4 threads, for each index one thread that build the index 
        // from the start to the middle, and one thread that build from the end
        // to the middle
        ////////////////////////////////////////////////////////////////////////
        let ones_bitvector_copy = bitvector.clone();
        let count_zeros = move || {
            let mut high_bits_index_ones = Vec::with_capacity(ones_bitvector_copy.len() >> QUANTUM_LOG2);
            let mut number_of_ones = 0;
            for (i, mut word) in ones_bitvector_copy.iter().cloned().enumerate() {
                while word != 0 {
                    // Get the bit position of the current one
                    let idx = (i << WORD_SHIFT) as usize + word.trailing_zeros() as usize;

                    // write the index
                    if number_of_ones & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                        high_bits_index_ones.push(idx as usize);
                    }

                    // Clean the one so that we can get to the next one.
                    word &= word - 1;
                    number_of_ones += 1;
                }
            }
            (number_of_ones, high_bits_index_ones)
        };
        
        // spawn a new thread if we can parallellize, otherwise just call it
        // sequentially
        #[cfg(feature="par_iter")]
        let ones_counter = std::thread::spawn(count_zeros);

        let mut high_bits_index_zeros = Vec::with_capacity(bitvector.len() >> QUANTUM_LOG2);
        let mut number_of_zeros = 0;
        for (i, mut word) in bitvector.iter().cloned().enumerate() {
            while word != usize::MAX {
                // Get the bit position of the current one
                let idx = (i << WORD_SHIFT) as usize + word.trailing_ones() as usize;

                // write the index
                if number_of_zeros & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                    high_bits_index_zeros.push(idx as usize);
                }

                // set the zero so that we can get to the next zero.
                word |= word + 1;
                number_of_zeros += 1;
            }
        }
        #[cfg(feature="par_iter")]
        let (number_of_ones, high_bits_index_ones) = ones_counter.join().unwrap();
        #[cfg(not(feature="par_iter"))]
        let (number_of_ones, high_bits_index_ones) = count_zeros();

        let bitvector = Arc::try_unwrap(bitvector).unwrap();

        SparseIndex{
            len: (bitvector.len() << WORD_SHIFT) as usize,
            number_of_zeros,
            number_of_ones,
            high_bits: bitvector,
            high_bits_index_zeros,
            high_bits_index_ones,
            phantom: core::marker::PhantomData::default(),
        }
    }
}
