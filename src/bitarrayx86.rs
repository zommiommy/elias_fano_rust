use crate::traits::{CoreIoError, MemoryFootprint, ReadBit, WriteBit};
use crate::utils::{fast_log2_ceil, power_of_two_to_mask};
use crate::codes::{CodeUnary, CodeFixedLength};
use crate::constants::*;
use core::mem::size_of;
use core::intrinsics::unlikely;
use alloc::vec::Vec;

use core::arch::x86_64::{
    __m128i,
    _mm_loadu_si128,
    _mm_cvtsi128_si64x,
    _mm_shuffle_epi8, // we cannot use _mm_slli_si128 
                      // because the shift has to be an immediate 
};

/// Optimized BitArray that exploits SSE instructions for faster reads
pub struct BitArrayX86_64 {
    /// The actual word reader / writer
    pub data: Vec<u64>,
    /// Index that keeps track in which word we currently are
    pub word_index: usize,
    /// Index that keeps track in which bit we are in the current word
    pub bit_index: usize,
}

impl MemoryFootprint for BitArray {
    fn total_size(&self) -> usize {
        self.data.total_size()
        + 2 * size_of::<usize>()
    }
}

impl BitArray {
    /// Create a new empty bitarray
    pub fn new() -> BitArray {
        BitArray{
            data: vec![0],
            word_index: 0,
            bit_index: 0,
        }
    }

    /// Create a new empty bitarray that will be able to write `capacity` bits
    /// without having to allocate memory.
    pub fn with_capacity(capacity: usize) -> BitArray {
        let mut data = Vec::with_capacity(capacity / (8 * size_of::<usize>()));
        data.push(0);
        
        BitArray{
            data,
            word_index: 0,
            bit_index: 0,
        }
    }

    #[inline]
    /// Destroy the BitArray wrapper and return the inner backend
    pub fn into_inner(self) -> Vec<usize> {
        self.data
    }
}

/// Optimized implementation that exploit the fact that all the data is already
/// in memory
impl CodeFixedLength for BitArray {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    /// 
    /// The expected latency is X cycles:
    /// ```asm
    /// 
    /// ```
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize, CoreIoError> {
        Ok(unsafe {
            let dword = _mm_loadu_si128(self.data.as_ptr());
            let aligned = _mm_shuffle_epi8(dword, self.bit_index >> 3);
            _mm_cvtsi128_si64x(aligned) >> (self.bit_index & 7)
        } as usize)
    }

    #[inline]
    /// Write `value` using `number_of_bits` in the stream.
    fn write_fixed_length(&mut self, number_of_bits: usize, value: usize) -> Result<(), CoreIoError> {
        debug_assert!(number_of_bits >= fast_log2_ceil(value), "value: {} n: {}", value, number_of_bits);
        // Compute how many bits we are going to write to each word
        let space_left = WORD_BIT_SIZE - self.bit_index;
        let first_word_number_of_bits = number_of_bits.min(space_left as usize);
        let second_word_number_of_bits = number_of_bits - first_word_number_of_bits;

        // this solve the assumptions in read_bits that we always have an extra word
        if self.word_index >= self.data.len() - 1 {
            self.data.resize(self.data.len() + 1, 0);
        }

        // write the data in the first word
        let first_word_bits = value & power_of_two_to_mask(first_word_number_of_bits as usize);
        self.data[self.word_index] |= first_word_bits.checked_shl(self.bit_index as u32).unwrap_or(0);

        // write the data in the second word
        let second_word_bits = (value >> first_word_number_of_bits) 
            & power_of_two_to_mask(second_word_number_of_bits as usize);
        self.data[self.word_index + 1] |= second_word_bits;

        // Update the pointers to after where we wrote
        self.skip_bits(number_of_bits as usize)
    }
}