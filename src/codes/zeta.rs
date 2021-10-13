use crate::utils::fast_log2;
use super::BitStream;

impl BitStream {

    #[inline]
    pub fn read_zeta<const K: u64>(&mut self) -> u64 {
        // on x86_64 this constant multiplication will be converted
        // to a LEA instruction which is MUCH faster than a MUL
        let len = self.read_unary();

        // read `len` blocks of `K` bits
        self.read_bits(K * len)
    }

    #[inline]
    pub fn write_zeta<const K: u64>(&mut self, value: u64) {
        // compute how many blocks of `K` bits we are going to use
        let number_of_blocks_to_write = (fast_log2(value) + K - 1) / K;
        self.write_unary(number_of_blocks_to_write);

        // write `prefix` blocks of `K` bits
        self.write_bits(K * number_of_blocks_to_write, value);
    }
}

#[cfg(test)]
mod test_zeta {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_zeta_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            bs.write_zeta::<4>(i);
        }
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_zeta::<4>());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_zeta_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            bs.write_zeta::<3>(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_zeta::<3>());
        }
    }
}