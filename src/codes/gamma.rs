use crate::utils::fast_log2_floor;
use super::BitStream;

/// Optimal for Zipf of exponent 2
/// Elias’ γ universal coding of x ∈ N+ is obtained by representing x in binary
// preceded by a unary representation of its length (minus one).
// More precisely, to represent x we write in unary floor(log(x)) and then in
// binary x - 2^ceil(log(x)) (on floor(log(x)) bits)
impl BitStream {

    // TODO FIX THIS SHIT
    #[inline]
    pub fn read_gamma(&mut self) -> u64 {
        let len = self.read_unary();
        self.read_bits(len) + (1 << len) - 1
    }

    #[inline]
    pub fn write_gamma(&mut self, mut value: u64) {
        value += 1;
        let number_of_blocks_to_write = fast_log2_floor(value);
        // remove the most significant 1
        let short_value = value - (1 << number_of_blocks_to_write);
        // TODO this can be optimized 
        // Write the code
        self.write_unary(number_of_blocks_to_write);
        self.write_bits(number_of_blocks_to_write, short_value);
    }

    pub fn size_gamma(&mut self, mut value: u64) -> u64 {
        value += 1;
        let number_of_blocks_to_write = fast_log2_floor(value);
        self.size_unary(number_of_blocks_to_write) 
            + self.size_bits(number_of_blocks_to_write)
    }
}

#[cfg(test)]
mod test_gamma {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_gamma_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            let idx = bs.tell();
            bs.write_gamma(i);
            assert_eq!(bs.tell(), idx + bs.size_gamma(i) as usize);
        }
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_gamma());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_gamma_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            let idx = bs.tell();
            bs.write_gamma(i);
            assert_eq!(bs.tell(), idx + bs.size_gamma(i) as usize);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_gamma());
        }
    }
}