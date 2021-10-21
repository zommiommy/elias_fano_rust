use crate::utils::{
    fast_log2_floor,
    fast_pow_2,
};
use super::BitStream;


impl BitStream {

    #[inline]
    pub fn read_zeta<const K: u64>(&mut self) -> u64 {
        let h = self.read_unary();
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);
        let r = self.read_minimal_binary(u - l);
        fast_pow_2(h * K) + r - 1
    }

    #[inline]
    pub fn write_zeta<const K: u64>(&mut self, mut value: u64) {
        value += 1;
        let h = fast_log2_floor(value) / K; // wtf
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);

        debug_assert!(l <= value, "{} <= {}", l, value);
        debug_assert!(value < u, "{} < {}", value, u);

        self.write_unary(h);
        self.write_minimal_binary( value - fast_pow_2(h * K),  u - l);
    }

    pub fn size_zeta<const K: u64>(&mut self, mut value: u64) -> u64 {
        value += 1;
        let h = fast_log2_floor(value) / K; // wtf
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);
        self.size_unary(h) 
            + self.size_minimal_binary(value - fast_pow_2(h * K),  u - l)
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
            let idx = bs.tell();
            bs.write_zeta::<3>(i);
            assert_eq!(bs.tell(), idx + bs.size_zeta::<3>(i) as usize);
        }
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_zeta::<3>());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_zeta_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            let idx = bs.tell();
            bs.write_zeta::<3>(i);
            assert_eq!(bs.tell(), idx + bs.size_zeta::<3>(i) as usize);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_zeta::<3>());
        }
    }
}