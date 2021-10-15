use crate::utils::fast_log2;
use super::BitStream;

impl BitStream {

    #[inline]
    pub fn read_golomb<const B: u64>(&mut self) -> u64 {
        let blocks_count = self.read_unary();
        blocks_count * B + self.read_bits(fast_log2(B))
    }

    #[inline]
    pub fn write_golomb<const B: u64>(&mut self, value: u64) {
        self.write_unary(value / B);
        self.write_bits( fast_log2(B), value % B);
    }
}

#[cfg(test)]
mod test_golomb {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_golomb_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            bs.write_golomb::<8>(i);
        }
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_golomb::<8>());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_golomb_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            bs.write_golomb::<8>(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_golomb::<8>());
        }
    }
}