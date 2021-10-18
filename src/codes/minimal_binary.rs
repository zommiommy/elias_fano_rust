use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use super::BitStream;

/// Huffman Optimal code for uniform distribution
impl BitStream {

    #[inline]
    pub fn read_minimal_binary(&mut self, max: u64) -> u64 {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let n = self.read_bits(l) ;
        let scarto = fast_pow_2(u) - max; 
        
        if n  < scarto {
            return n;
        } 
        // rewind to read the code again
        self.rewind(l as _);
        // decode the value
        let r = self.read_bits(u);

        if r < fast_pow_2(l) {
            r
        } else {
            r - scarto
        }
    }

    #[inline]
    pub fn write_minimal_binary(&mut self, value: u64, max: u64) {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;

        if value < scarto {
            self.write_bits(l, value);
        } else if value < fast_pow_2(l) {
            self.write_bits(u, value);
        } else {
            self.write_bits(u, value + scarto);
        }
    }
}

#[cfg(test)]
mod test_minimal_binary {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_minimal_binary_forward() {
        let mut bs = BitStream::new();
        let max = 1_000;
        for i in 0..max {
            bs.write_minimal_binary(i, max);
        }
        bs.seek(0);
        for i in 0..max {
            assert_eq!(i, bs.read_minimal_binary(max));
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_minimal_binary_backward() {
        let mut bs = BitStream::new();
        let max = 1_000;
        for i in (0..max).rev() {
            bs.write_minimal_binary(i, max);
        }
        bs.seek(0);
        for i in (0..max).rev() {
            assert_eq!(i, bs.read_minimal_binary(max));
        }
    }
}