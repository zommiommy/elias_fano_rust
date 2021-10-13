//!
//! 
//! |       alpha |         Code |
//! |-------------|--------------|
//! | < 1.06      | Elia's Delta |
//! | [1.06,1.08] | zeta<6>      |
//! | [1.08,1.11] | zeta<5>      |
//! | [1.11,1.16] | zeta<4>      |
//! | [1.16,1.27] | zeta<3>      |
//! | [1.27,1.57] | zeta<2>      |
//! | [1.57,1.2]  | Elia's Gamma |
//! 

use crate::utils::power_of_two_to_mask;
mod unary;
mod gamma;
mod zeta;

pub struct BitStream {
    data: Vec<u64>,
    word_index: usize,
    bit_index: usize,
}

impl crate::traits::MemoryFootprint for BitStream {
    fn total_size(&self) -> usize {
        std::mem::size_of::<u64>() * self.data.len()
        + 4 * std::mem::size_of::<usize>()
    }
}

impl BitStream {
    pub fn new() -> BitStream {
        BitStream{
            data: vec![0],
            word_index: 0,
            bit_index: 0,
        }
    }

    #[inline]
    pub fn seek(&mut self, index: usize) {
        self.word_index = index >> 6;
        self.bit_index = index & 63;
    }

    #[inline]
    pub fn tell(&mut self) -> usize {
        (self.word_index << 6) | self.bit_index
    }

    #[inline]
    /// Seek forward by `offset` bits 
    pub fn skip(&mut self, offset: usize) {
        self.bit_index += offset;
        self.word_index += self.bit_index >> 6;
        self.bit_index &= 63;
    }

    #[inline]
    /// Clean the BitStream, this can be used to re-use the allocations
    pub fn clear(&mut self) {
        self.word_index = 0;
        self.bit_index = 0;
        self.data.clear();
    }

    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    pub fn read_bits(&mut self, number_of_bits: u64) -> u64 {
        // read the data from the current word
        let code = self.data[self.word_index] >> self.bit_index;

        // compute how many bits did we read and how many are left
        let bits_read = 64 - self.bit_index as u64;

        // read the next word, this implies that we will always have one
        // extra word in the data stream
        let next = self.data[self.word_index + 1];

        // concatenate the data from the two words
        let aligned_data = code | (next.checked_shl(bits_read as u32).unwrap_or(0));

        // clear off the excess bits.
        // we shall keep only the lower `number_of_bits` bits.
        let result = aligned_data & power_of_two_to_mask(number_of_bits as usize);

        // Update the pointers to where we read
        self.skip(number_of_bits as usize);

        result
    }

    #[inline]
    /// Write `value` using `number_of_bits` in the stream.
    pub fn write_bits(&mut self, number_of_bits: u64, value: u64) {
        // Compute how many bits we are going to write to each word
        let space_left = 64 - self.bit_index;
        let first_word_number_of_bits = number_of_bits.min(space_left as u64);
        let second_word_number_of_bits = number_of_bits - first_word_number_of_bits;

        // this solve the assumptions in read_bits that we always have an extra word
        self.data.resize(self.data.len() + 1, 0);

        let first_word_bits = value & power_of_two_to_mask(first_word_number_of_bits as usize);
        self.data[self.word_index] |= first_word_bits.checked_shl(self.bit_index as u32).unwrap_or(0);

        let second_word_bits = (value >> first_word_number_of_bits) & power_of_two_to_mask(second_word_number_of_bits as usize);
        self.data[self.word_index + 1] |= second_word_bits;

        self.skip(number_of_bits as usize);
    }
}


#[cfg(test)]
mod test_bitstream_utils {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_read_bits() {
        let mut bs = BitStream::new();
        for i in 0..10_000 {
            bs.write_bits(14, i);
        }
        bs.seek(0);
        for i in 0..10_000 {
            assert_eq!(i, bs.read_bits(14));
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_read_bits_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            bs.write_bits(14, i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_bits(14));
        }
    }
}