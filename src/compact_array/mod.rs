use crate::utils::power_of_two_to_mask;
use std::sync::atomic::AtomicU64;
mod primitives;

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
    data: Vec<u64>,
    word_size: u64,
    word_mask: u64,
}

impl crate::traits::MemoryFootprint for CompactArray {
    fn total_size(&self) -> usize {
        std::mem::size_of::<u64>() * (2 + self.data.len())
        + 2 * std::mem::size_of::<usize>()
    }
}

impl CompactArray {
    pub fn new(word_size: u64) -> CompactArray {
        CompactArray{
            data: Vec::new(),
            word_mask: power_of_two_to_mask(word_size as usize),
            word_size,
        }
    }

    pub fn with_capacity(word_size: u64, capacity: usize) -> CompactArray {
        CompactArray{
            data: vec![0; get_vec_size(word_size, capacity) as usize],
            word_mask: power_of_two_to_mask(word_size as usize),
            word_size,
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    #[inline]
    /// Read a value from the compact array
    pub fn read(&self, index: u64) -> u64 {
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
    pub fn write(&mut self, index: u64, value: u64) {
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
    pub fn concurrent_write(&self, index: u64, value: u64) {
        let ptr = unsafe {
            std::mem::transmute::<&Vec<u64>, &Vec<AtomicU64>>(&self.data)
        };
        concurrent_write(
            ptr, 
            index, 
            value, 
            self.word_size
        );
    }

    #[inline]
    pub fn word_size(&self) -> u64 {
        self.word_size
    }

    #[inline]
    pub fn word_mask(&self) -> u64 {
        self.word_mask
    }

    pub fn size(&self) -> usize {
        (self.data.capacity() * std::mem::size_of::<u64>()) + 2 * std::mem::size_of::<Vec<u64>>()
    }
}