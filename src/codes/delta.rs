use crate::utils::fast_log2_ceil;
use super::BitStream;

// Elias’ δ universal coding of x ∈ N+ is obtained by representing x in binary
// preceded by a representation of its length in γ.
impl BitStream {

    #[inline]
    pub fn read_delta(&mut self) -> u64 {
        let len = self.read_gamma();
        self.read_bits(len)
    }

    #[inline]
    pub fn write_delta(&mut self, value: u64) {
        // TODO!: figure out if the +1 is actually needed
        let number_of_blocks_to_write = fast_log2_ceil(value + 1);
        self.write_gamma(number_of_blocks_to_write);
        self.write_bits(number_of_blocks_to_write, value);
    }

    pub fn size_delta(&mut self, value: u64) -> u64 {
        let number_of_blocks_to_write = fast_log2_ceil(value + 1);
        self.size_gamma(number_of_blocks_to_write) 
            + self.size_bits(number_of_blocks_to_write)
    }
}

#[cfg(test)]
mod test_delta {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_delta_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            let idx = bs.tell();
            bs.write_delta(i);
            assert_eq!(bs.tell(), idx + bs.size_delta(i) as usize);
        }
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_delta());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_delta_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            let idx = bs.tell();
            bs.write_delta(i);
            assert_eq!(bs.tell(), idx + bs.size_delta(i) as usize);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_delta());
        }
    }
}