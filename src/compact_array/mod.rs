//! A simple array that stores conescutively values using a fixed length binary 
//! encoding
//! 
use crate::utils::power_of_two_to_mask;
use core::sync::atomic::AtomicUsize;
use core::mem::size_of;
mod primitives;
use alloc::vec::Vec;

#[cfg(feature="fuzz")]
pub use primitives::*;
#[cfg(not(feature="fuzz"))]
use primitives::*;

#[derive(Debug, Clone, PartialEq)]
/// A simple compact array with custom word-length.
/// This is the trivial representation where 
/// the i-th value can be written / read from the bits
/// (word_size * i..word_size * (i + 1))
pub struct CompactArray{
    data: Vec<usize>,
    word_size: usize,
    word_mask: usize,
}

impl crate::traits::MemoryFootprint for CompactArray {
    fn total_size(&self) -> usize {
        size_of::<usize>() * 2 + self.data.total_size()
    }
}

impl CompactArray {
    /// Create a new empty CompactArray
    pub fn new(word_size: usize) -> CompactArray {
        CompactArray{
            data: Vec::new(),
            word_mask: power_of_two_to_mask(word_size as usize),
            word_size,
        }
    }

    /// Create a new empty CompactArray that can write up to `capacity` elements
    /// without the need for re-allocation 
    pub fn with_capacity(word_size: usize, capacity: usize) -> CompactArray {
        CompactArray{
            data: vec![0; get_vec_size(word_size, capacity) as usize],
            word_mask: power_of_two_to_mask(word_size as usize),
            word_size,
        }
    }

    /// Reduce any spurious memory preventively allocated in vectors 
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    #[inline]
    /// Read a value from the compact array
    pub fn read(&self, index: usize) -> usize {
        #[cfg(not(feature = "unsafe"))]
        return safe_read(&self.data, index, self.word_size);
        #[cfg(feature = "unsafe")]
        return unsafe_read(&self.data, index, self.word_size);
    }

    #[inline]
    /// Write a value from the compact array.
    /// 
    /// # Safety
    /// This assumes that `value` fit in `word_size` bits.
    pub fn write(&mut self, index: usize, value: usize) {
        #[cfg(not(feature = "unsafe"))]
        safe_write(&mut self.data, index, value, self.word_size);
        #[cfg(feature = "unsafe")]
        unsafe_write(&mut self.data, index, value, self.word_size, );
    }

    #[inline]
    /// Write a value from the compact array in a thread safe way
    /// 
    /// # Safety
    /// This assumes that `value` fit in `word_size` bits.
    pub fn concurrent_write(&self, index: usize, value: usize) {
        let ptr = unsafe {
            core::mem::transmute::<&Vec<usize>, &Vec<AtomicUsize>>(&self.data)
        };
        concurrent_write(
            ptr, 
            index, 
            value, 
            self.word_size
        );
    }

    #[inline]
    /// Return the word size associated with the current CompactArray
    pub fn word_size(&self) -> usize {
        self.word_size
    }

    #[inline]
    /// Return the word mask associated with the current CompactArray
    pub fn word_mask(&self) -> usize {
        self.word_mask
    }
}