use crate::utils::fast_log2;
use super::BitStream;

impl BitStream {

    #[inline]
    pub fn read_gamma(&mut self) -> u64 {
        let len = self.read_unary();
        self.read_bits(len)
    }

    #[inline]
    pub fn write_gamma(&mut self, value: u64) {
        let number_of_blocks_to_write = fast_log2(value);
        self.write_unary(number_of_blocks_to_write);
        self.write_bits(number_of_blocks_to_write, value);
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
            bs.write_gamma(i);
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
            bs.write_gamma(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_gamma());
        }
    }
}