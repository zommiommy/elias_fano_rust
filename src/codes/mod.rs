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

mod delta;
mod fixed_length;
mod gamma;
mod golomb;
mod minimal_binary;
mod minimal_binary_bv;
mod unary;
mod var_length;
mod zeta;
pub use golomb::compute_optimal_golomb_block_size;

pub struct BitStream {
    pub data: Vec<u64>,
    pub word_index: usize,
    pub bit_index: usize,
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

    pub fn with_capacity(capacity: usize) -> BitStream {
        let mut data = Vec::new();
        data.resize(capacity, 0);
        BitStream{
            data,
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
        // is this faster than a tell + seek?
        self.bit_index += offset;
        self.word_index += self.bit_index >> 6;
        self.bit_index &= 63;
    }    
    
    #[inline]
    /// Seek backward by `offset` bits 
    pub fn rewind(&mut self, offset: usize) {
        let index = self.tell() - offset;
        self.seek(index);
    }

    #[inline]
    /// Clean the BitStream, this can be used to re-use the allocations
    pub fn clear(&mut self) {
        self.word_index = 0;
        self.bit_index = 0;
        self.data.clear();
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

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_bitstream() {
        let mut bs = BitStream::new();
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
        let mut bs = BitStream::new();
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