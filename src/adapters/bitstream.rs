
use crate::traits::{Read, ReadBit, Write, WriteBit, MemoryFootprint};
use crate::constants::*;
use crate::utils::{fast_log2_ceil, power_of_two_to_mask};
use crate::codes::{CodeUnary, CodeFixedLength};
use core::mem::size_of;
use core::intrinsics::unlikely;

/// A general bitstream wrapper over some word reader and writers.
/// The goal of this wrapper is to be able to provide a generic Monad:
/// `T: Read + Write => R: ReadBit + WriteBit`
pub struct BitStream<BACKEND: Write + Read + MemoryFootprint = alloc::vec::Vec<usize>> {
    /// The actual word reader / writer
    pub backend: BACKEND,
    /// A small buffer to be able to handle unaligned reads or
    /// writes
    pub buffer: [usize; 2],
    /// Index that keeps track in which word we currently are
    pub word_index: usize,
    /// Index that keeps track in which bit we are in the current word
    pub bit_index: usize,
}

impl MemoryFootprint for BitStream {
    fn total_size(&self) -> usize {
        self.backend.total_size()
        + 4 * size_of::<usize>()
    }
}

impl<BACKEND: Write + Read + MemoryFootprint> BitStream<BACKEND> {
    pub fn new(backend: BACKEND) -> BitStream {
        BitStream{
            backend,
            buffer: [0, 0],
            word_index: 0,
            bit_index: 0,
        }
    }

    #[inline]
    pub fn seek(&mut self, index: usize) {
        self.word_index = index >> WORD_SHIFT;
        self.bit_index = index & WORD_BIT_SIZE_MASK;
    }

    #[inline]
    pub fn tell(&mut self) -> usize {
        (self.word_index << WORD_SHIFT) | self.bit_index
    }

    #[inline]
    /// Seek forward by `offset` bits 
    pub fn skip(&mut self, offset: usize) {
        // is this faster than a tell + seek?
        self.bit_index += offset;
        self.word_index += self.bit_index >> WORD_SHIFT;
        self.bit_index &= WORD_BIT_SIZE_MASK;
    }    
    
    #[inline]
    /// Seek backward by `offset` bits 
    pub fn rewind(&mut self, offset: usize) {
        let index = self.tell() - offset;
        self.seek(index);
    }

    #[inline]
    /// Destroy the Bitstream wrapper and return the inner backend
    pub fn into_inner(self) {
        self.backend
    }

    #[inline]
    /// Read a single bit
    pub fn read_bit(&mut self) -> bool {
        // TODO!; optimize?
        let res = (self.data[self.word_index] >> self.bit_index) & 1;
        self.skip(1);
        res != 0
    }

    #[inline]
    /// Read a single bit
    pub fn write_bit(&mut self, value: bool) {
        // TODO!: optimize brenchless?
        if value {
            self.data[self.word_index] |= 1 << self.bit_index;
        }
        self.skip(1);
        if self.word_index >= self.data.len() - 1 {
            self.data.resize(self.word_index + 1, 0);
        }
    }
}


#[cfg(test)]
mod test_bitstream {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_bitstream() {
        let mut backend = <Vec<u64>>::new();
        let mut bs = BitStream::new(backend);
        assert_eq!(bs.tell(), 0);
        bs.write_bits(10, 7);
        assert_eq!(bs.tell(), 10);
        bs.rewind(10);
        assert_eq!(bs.tell(), 0);
        assert_eq!(bs.read_bits(10), 7);
        bs.seek(1);
        assert_eq!(bs.tell(), 1);
        bs.skip(3);
        assert_eq!(bs.tell(), 4);   
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_bitstream_() {
        let mut backend = <Vec<u64>>::new();
        let mut bs = BitStream::new(backend);
        for _ in 0..513 {
            bs.write_bit(true);
        }
        for _ in 0..513 {
            bs.write_bit(false);
        }
        for i in 0..513 {
            bs.write_bit(i % 2 == 0);
        }
        bs.seek(0);
        for _ in 0..513 {
            assert_eq!(bs.read_bit(), true);
        }
        for _ in 0..513 {
            assert_eq!(bs.read_bit(), false);
        }
        for i in 0..513 {
            assert_eq!(bs.read_bit(), i % 2 == 0);
        }
    }
}


/// Optimal for gemetric distribution of ratio 1/2
impl CodeUnary for BitStream {
    #[inline]
    fn read_unary(&mut self) -> usize {
        let mut res = 0;
        loop {
            let word = self.data[self.word_index] >> self.bit_index;
            let x = word.trailing_zeros() as usize;
            // if the code is not finished, continue to the next word
            let bound = (WORD_BIT_SIZE - self.bit_index) as usize;
            if unlikely(x >= bound) {
                self.word_index += 1;
                self.bit_index = 0;
                res += bound;
                continue
            }

            // the code finish here
            self.skip(1 + x as usize);
            return x + res;
        }
    }

    #[inline]
    fn write_unary(&mut self, value: usize) {
        // Update the reminder
        let idx = value + self.tell();

        let bit_index  = idx & WORD_BIT_SIZE_MASK; 
        let word_index = idx >> WORD_SHIFT;

        self.data.resize(word_index + 2, 0);

        // Write the bit
        self.data[word_index] |= 1 << bit_index;
        
        self.seek(idx + 1);
    }

}



/// Optimal for uniform distribution
impl CodeFixedLength for BitStream {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    fn read_bits(&mut self, number_of_bits: usize) -> usize {
        // read the data from the current word
        let code = self.data[self.word_index] >> self.bit_index;

        // compute how many bits did we read and how many are left
        let bits_read = WORD_BIT_SIZE - self.bit_index;

        // read the next word, this implies that we will always have one
        // extra word in the data stream
        let next = self.data[self.word_index + 1];

        // concatenate the data from the two words
        let aligned_data = code | (next.checked_shl(bits_read as u32).unwrap_or(0));

        // clear off the excess bits.
        // we shall keep only the lower `number_of_bits` bits.
        let result = aligned_data & power_of_two_to_mask(number_of_bits);

        // Update the pointers to where we read
        self.skip(number_of_bits);

        result
    }

    #[inline]
    /// Write `value` using `number_of_bits` in the stream.
    fn write_bits(&mut self, number_of_bits: usize, value: usize) {
        debug_assert!(number_of_bits >= fast_log2_ceil(value), "value: {} n: {}", value, number_of_bits);
        // Compute how many bits we are going to write to each word
        let space_left = WORD_BIT_SIZE - self.bit_index;
        let first_word_number_of_bits = number_of_bits.min(space_left);
        let second_word_number_of_bits = number_of_bits - first_word_number_of_bits;

        // this solve the assumptions in read_bits that we always have an extra word
        if self.word_index >= self.data.len() - 1 {
            self.data.resize(self.data.len() + 1, 0);
        }

        // write the data in the first word
        let first_word_bits = value & power_of_two_to_mask(first_word_number_of_bits as usize);
        self.data[self.word_index] |= first_word_bits.checked_shl(self.bit_index as u32).unwrap_or(0);

        // write the data in the second word
        let second_word_bits = (value >> first_word_number_of_bits) & power_of_two_to_mask(second_word_number_of_bits as usize);
        self.data[self.word_index + 1] |= second_word_bits;

        // Update the pointers to after where we wrote
        self.skip(number_of_bits as usize);
    }
}