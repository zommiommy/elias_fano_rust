use crate::utils::fast_log2_ceil;
use super::BitStream;


#[inline]
/// Given the ratio `p` of a geometric distribution
/// compute the optimal golomb block size
pub fn compute_optimal_golomb_block_size(p: f64) -> u64 {
    (-(2.0 - p).log2() / (1.0 - p).log2()).ceil() as u64
}


/// Optimal for gemetric distribution of ratio:
/// $$\frac{1}{\sqrt^b{2}$$
impl BitStream {

    #[inline]
    pub fn read_golomb<const B: u64>(&mut self) -> u64 {
        let blocks_count = self.read_unary();
        blocks_count * B + self.read_bits(fast_log2_ceil(B))
    }

    #[inline]
    pub fn write_golomb<const B: u64>(&mut self, value: u64) {
        self.write_unary(value / B);
        self.write_bits( fast_log2_ceil(B), value % B);
    }

    #[inline]
    pub fn size_golomb<const B: u64>(&mut self, value: u64) -> u64 {
        self.size_unary(value / B)
            + self.size_bits( fast_log2_ceil(B))
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
            let idx = bs.tell();
            bs.write_golomb::<8>(i);
            assert_eq!(bs.tell(), idx + bs.size_golomb::<8>(i) as usize);
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
            let idx = bs.tell();
            bs.write_golomb::<8>(i);
            assert_eq!(bs.tell(), idx + bs.size_golomb::<8>(i) as usize);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_golomb::<8>());
        }
    }
}