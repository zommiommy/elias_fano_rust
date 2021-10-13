use super::{
    constants::*,
    utils::*,   
};
use std::ops::Range;
use std::sync::Arc;

mod memory;
pub use memory::*;
mod iter;
pub use iter::*;
mod iter_double_ended;
pub use iter_double_ended::*;
mod getters;
mod concurrent;
pub use concurrent::*;

#[derive(Clone, Debug)]
/// This structure is efficient for **DENSE** bitvectors
/// Sparse Index which stores the position of every q-th 1 and 0
pub struct SparseIndex<const QUANTUM_LOG2: usize> {
    pub high_bits: Vec<u64>,
    pub high_bits_index_zeros: Vec<u64>,
    pub high_bits_index_ones: Vec<u64>,
    pub number_of_ones: u64,
    pub number_of_zeros: u64,
    pub len: u64,

    /// Make rust happy about having a fixed index size for a given structure
    phantom: std::marker::PhantomData<[(); QUANTUM_LOG2]>,
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
            phantom: std::marker::PhantomData::default(),
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
            phantom: std::marker::PhantomData::default(),
        }
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

        if self.len & WORD_MASK == 0{
            self.high_bits.push(0);
        }

        if value {
            let last_idx = self.high_bits.len() - 1;
            let mut code = self.high_bits[last_idx];
            code |= 1 << (self.len & WORD_MASK);
            self.high_bits[last_idx] = code;
        }

        self.len += 1;
    }

    /// Take the given bit-vector and build the indices on it.
    pub fn from_vec(bitvector: Vec<u64>) -> SparseIndex<QUANTUM_LOG2> {

        let bitvector = Arc::new(bitvector);

        // The following two steps are independant so we could parallelize them
        // using two separate threads
        // moreover, if we know in advance the number of ones and zeros in the bitvector
        // we can use 4 threads, for each index one thread that build the index 
        // from the start to the middle, and one thread that build from the end
        // to the middle
        ////////////////////////////////////////////////////////////////////////
        let ones_bitvector_copy = bitvector.clone();
        let ones_counter = std::thread::spawn(move || {
            let mut high_bits_index_ones = Vec::with_capacity(ones_bitvector_copy.len() >> QUANTUM_LOG2);
            let mut number_of_ones = 0;
            for (i, mut word) in ones_bitvector_copy.iter().cloned().enumerate() {
                while word != 0 {
                    // Get the bit position of the current one
                    let idx = (i << WORD_SHIFT) as u64 + word.trailing_zeros() as u64;

                    // write the index
                    if number_of_ones & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                        high_bits_index_ones.push(idx as u64);
                    }

                    // Clean the one so that we can get to the next one.
                    word &= word - 1;
                    number_of_ones += 1;
                }
            }
            (number_of_ones, high_bits_index_ones)
        });

        let mut high_bits_index_zeros = Vec::with_capacity(bitvector.len() >> QUANTUM_LOG2);
        let mut number_of_zeros = 0;
        for (i, mut word) in bitvector.iter().cloned().enumerate() {
            while word != u64::MAX {
                // Get the bit position of the current one
                let idx = (i << WORD_SHIFT) as u64 + word.trailing_ones() as u64;

                // write the index
                if number_of_zeros & power_of_two_to_mask(QUANTUM_LOG2) == 0 {
                    high_bits_index_zeros.push(idx as u64);
                }

                // set the zero so that we can get to the next zero.
                word |= word + 1;
                number_of_zeros += 1;
            }
        }

        let (number_of_ones, high_bits_index_ones) = ones_counter.join().unwrap();

        let bitvector = Arc::try_unwrap(bitvector).unwrap();

        SparseIndex{
            len: (bitvector.len() << WORD_SHIFT) as u64,
            number_of_zeros,
            number_of_ones,
            high_bits: bitvector,
            high_bits_index_zeros,
            high_bits_index_ones,
            phantom: std::marker::PhantomData::default(),
        }
    }
}
