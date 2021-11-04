
use crate::traits::{CoreIoError, MemoryFootprint, ReadBit, WriteBit};
use crate::utils::{fast_log2_ceil, power_of_two_to_mask};
use crate::codes::{CodeUnary, CodeFixedLength};
use crate::constants::*;
use core::mem::size_of;
use core::intrinsics::unlikely;
use alloc::vec::Vec;

/// A general BitArrayBig wrapper over some word reader and writers.
/// This assumes that the words of memory are read and write in little-endian;
/// 
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
/// 
/// let mut ba = BitArrayBig::new();
/// 
/// // write a pattern of single bits to the array
/// for _ in 0..513 {
///     ba.write_bit(true);
/// }
/// for _ in 0..513 {
///     ba.write_bit(false);
/// }
/// for i in 0..513 {
///     ba.write_bit(i % 2 == 0);
/// }
/// 
/// // rewind completely the BitArrayBig
/// ba.seek_bits(0);
/// 
/// // Ensure that we read back exacly the same pattern
/// for _ in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), true);
/// }
/// for _ in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), false);
/// }
/// for i in 0..513 {
///     assert_eq!(ba.read_bit().unwrap(), i % 2 == 0);
/// }  
/// 
/// // rewind completely the BitArrayBig
/// ba.clear();
/// let max = 9;
/// for i in 0..1 << max {
///     ba.write_fixed_length(max, i).unwrap();
/// }  
/// ba.seek_bits(0);
/// for i in 0..1 << max {
///     assert_eq!(ba.read_fixed_length(max).unwrap(), i);
/// } 
/// 
/// // rewind completely the BitArrayBig
/// ba.clear();
/// for i in 0..513 {
///     ba.write_unary(i).unwrap();
/// }  
/// ba.seek_bits(0);
/// for i in 0..513 {
///     assert_eq!(ba.read_unary().unwrap(), i);
/// }  
/// ```
pub struct BitArrayBig {
    /// The actual word reader / writer
    pub data: Vec<usize>,
    /// Index that keeps track in which word we currently are
    pub word_index: usize,
    /// Index that keeps track in which bit we are in the current word
    pub bit_index: usize,
}

impl MemoryFootprint for BitArrayBig {
    fn total_size(&self) -> usize {
        self.data.total_size()
        + 2 * size_of::<usize>()
    }
}

impl BitArrayBig {
    /// Create a new empty bitarray
    pub fn new() -> BitArrayBig {
        BitArrayBig{
            data: vec![0],
            word_index: 0,
            bit_index: 0,
        }
    }

    /// Create a new empty bitarray that will be able to write `capacity` bits
    /// without having to allocate memory.
    pub fn with_capacity(capacity: usize) -> BitArrayBig {
        let mut data = Vec::with_capacity(capacity / (8 * size_of::<usize>()));
        data.push(0);
        
        BitArrayBig{
            data,
            word_index: 0,
            bit_index: 0,
        }
    }

    #[inline]
    /// Destroy the BitArrayBig wrapper and return the inner backend
    pub fn into_inner(self) -> Vec<usize> {
        self.data
    }

    /// Reset the vector keeping the memory allocation
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(0);
        self.word_index = 0;
        self.bit_index = 0;
    }
}

impl ReadBit for BitArrayBig {
    #[inline]
    /// Read a single bit
    fn read_bit(&mut self) -> Result<bool, CoreIoError> {
        // TODO!; optimize?
        let res = (self.data[self.word_index] << self.bit_index) & WORD_HIGHEST_BIT_MASK;
        self.skip_bits(1)?;
        Ok(res != 0)
    }

    #[inline]
    /// Seek to the given bit_index
    fn seek_bits(&mut self, bit_index: usize) -> Result<(), CoreIoError> {
        self.word_index = bit_index >> WORD_SHIFT;
        self.bit_index = bit_index & WORD_BIT_SIZE_MASK;
        Ok(())
    }

    #[inline]
    /// Return the current position (bit index) in the bit array
    fn tell_bits(&self) -> Result<usize, CoreIoError> {
        Ok((self.word_index << WORD_SHIFT) | self.bit_index)
    }

    #[inline]
    /// Overriding optimized version
    fn skip_bits(&mut self, bit_offset: usize) -> Result<(), CoreIoError> {
        // TODO!: is this faster than a tell + seek?
        self.bit_index += bit_offset;
        self.word_index += self.bit_index >> WORD_SHIFT;
        self.bit_index &= WORD_BIT_SIZE_MASK;
        Ok(())
    }    
}

impl WriteBit for BitArrayBig {
    #[inline]
    /// Read a single bit
    fn write_bit(&mut self, value: bool) -> Result<(), CoreIoError>{
        // TODO!: optimize brenchless?
        if value {
            self.data[self.word_index] |= WORD_HIGHEST_BIT_MASK >> self.bit_index;
        }
        self.skip_bits(1)?;
        if self.word_index >= self.data.len() - 1 {
            self.data.resize(self.word_index + 1, 0);
        }

        Ok(())
    }

    #[inline]
    /// For a bitarray there is not need to flush anything
    fn flush_bits(&mut self) {}
}

/// Optimal for gemetric distribution of ratio 1/2
impl CodeUnary for BitArrayBig {
    #[inline]
    fn read_unary(&mut self) -> Result<usize, CoreIoError>  {
        let mut res = 0;
        loop {
            let word = self.data[self.word_index] << self.bit_index;
            let x = word.leading_zeros() as usize;
            // if the code is not finished, continue to the next word
            let bound = WORD_BIT_SIZE - self.bit_index;
            if unlikely(x >= bound) {
                self.word_index += 1;
                self.bit_index = 0;
                res += bound;
                continue
            }

            // the code finish here
            self.skip_bits(1 + x)?;
            return Ok(x + res);
        }
    }

    #[inline]
    fn write_unary(&mut self, value: usize) -> Result<(), CoreIoError> {
        // Update the reminder
        let idx = value + self.tell_bits()?;

        let bit_index  = idx & WORD_BIT_SIZE_MASK; 
        let word_index = idx >> WORD_SHIFT;

        self.data.resize(word_index + 2, 0);

        // Write the bit
        self.data[word_index] |= WORD_HIGHEST_BIT_MASK >> bit_index;
        
        self.seek_bits(idx + 1)
    }

}


/// Optimized implementation that exploit the fact that all the data is already
/// in memory
impl CodeFixedLength for BitArrayBig {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize, CoreIoError> {
        // read the data from the current word
        let code = self.data[self.word_index] << self.bit_index;
        // read the next word, this implies that we will always have one
        // extra word in the data stream
        let next = self.data[self.word_index + 1];

        // compute how many bits did we read and how many are left
        let bits_read = WORD_BIT_SIZE - self.bit_index as usize;

        // concatenate the data from the two words
        let aligned_data = code | (
            next.checked_shr(bits_read as u32).unwrap_or(0)
        );

        // clear off the excess bits.
        // we shall keep only the lower `number_of_bits` bits.
        let result = aligned_data & power_of_two_to_mask(number_of_bits as _);

        // Update the pointers to where we read
        self.skip_bits(number_of_bits as usize)?;

        Ok(result)
    }

    #[inline]
    /// Write `value` using `number_of_bits` in the stream.
    fn write_fixed_length(&mut self, number_of_bits: usize, value: usize) -> Result<(), CoreIoError> {
        debug_assert!(number_of_bits >= fast_log2_ceil(value), "value: {} n: {}", value, number_of_bits);

        // We want to append a given number of bits to the stream, the values 
        // can span two words:
        //
        //  M  Word 1       L M       Word 2  L
        // |.................|.................|
        //              |....|.......|
        //               M  Bits    L
        //
        // So we need to compute how many bits we can write in the current Word
        // and then how many bits are left to write in the second one
        //
        // Then we can compose the words 
    

        // Compute how many bits we are going to write to each word
        let space_left = WORD_BIT_SIZE - self.bit_index;
        let first_word_number_of_bits = number_of_bits.min(space_left as usize);
        let second_word_number_of_bits = number_of_bits - first_word_number_of_bits;

        // write the data in the first word
        let first_word_bits = value 
            & power_of_two_to_mask(first_word_number_of_bits as usize);
        // write the data in the second word
        let second_word_bits = (value >> first_word_number_of_bits) 
            & power_of_two_to_mask(second_word_number_of_bits as usize);

        // this solve the assumptions in read_bits that we always have an extra word
        // and also we can avoid bound checking
        if self.word_index + 1 >= self.data.len() {
            self.data.resize(self.data.len() + 1, 0);
        }

        unsafe{
            *self.data.get_unchecked_mut(self.word_index) |= 
                first_word_bits << (self.bit_index & WORD_BIT_SIZE_MASK);

            *self.data.get_unchecked_mut(self.word_index + 1) |= second_word_bits;
        }

        // Update the pointers to after where we wrote
        self.skip_bits(number_of_bits as usize)
    }
}