use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use super::BitStream;


#[inline(always)]
fn reverse_bits(value: u64, number_of_bits: u64) -> u64 {
    value.reverse_bits() >> (64 - number_of_bits)
}

/// Huffman Optimal code for uniform distribution, as described by Boldi and Vigna
impl BitStream {

    #[inline]
    pub fn read_minimal_binary_bv(&mut self, max: u64) -> u64 {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let n = reverse_bits(self.read_bits(l), l);
        let scarto = fast_pow_2(u) - max; 
        
        if n  < scarto {
            return n;
        } 
        // rewind to read the code again
        self.rewind(l as _);
        // decode the value
        let r = reverse_bits(self.read_bits(u), u);
        r - scarto
    }

    #[inline]
    pub fn write_minimal_binary_bv(&mut self, value: u64, max: u64) {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;

        if value < scarto {
            self.write_bits(l, reverse_bits(value, l));
        } else {
            self.write_bits(u, reverse_bits(value + scarto, u));
        }
    }

    pub fn size_minimal_binary_bv(&mut self, value: u64, max: u64) -> u64 {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;
        
        if value < scarto {
            l
        } else {
            u
        }
    }
}

#[cfg(test)]
mod test_minimal_binary_bv {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        for i in 0..1_000 {
            assert_eq!(i, reverse_bits(reverse_bits(i, 10), 10));
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_minimal_binary_bv_forward() {
        let mut bs = BitStream::new();
        let max = 1_000;
        for i in 0..max {
            let idx = bs.tell();
            bs.write_minimal_binary_bv(i, max);
            assert_eq!(bs.tell(), idx + bs.size_minimal_binary_bv(i, max) as usize);
        }
        bs.seek(0);
        for i in 0..max {
            assert_eq!(i, bs.read_minimal_binary_bv(max));
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_minimal_binary_bv_backward() {
        let mut bs = BitStream::new();
        let max = 1_000;
        for i in (0..max).rev() {
            let idx = bs.tell();
            bs.write_minimal_binary_bv(i, max);
            assert_eq!(bs.tell(), idx + bs.size_minimal_binary_bv(i, max) as usize);
        }
        bs.seek(0);
        for i in (0..max).rev() {
            assert_eq!(i, bs.read_minimal_binary_bv(max));
        }
    }
}