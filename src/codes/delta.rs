use crate::utils::fast_log2_ceil;
use super::BitStream;

// Elias’ δ universal coding of x ∈ N+ is obtained by representing x in binary
// preceded by a representation of its length in γ.
impl BitStream {

    // TODO FIX THIS SHIT
    #[inline]
    pub fn read_delta(&mut self) -> u64 {
        let len = self.read_gamma();
        self.read_bits(len) - 1
    }

    #[inline]
    pub fn write_delta(&mut self, mut value: u64) {
        value += 1;
        let number_of_blocks_to_write = fast_log2_ceil(value);
        self.write_gamma(number_of_blocks_to_write);
        self.write_bits(number_of_blocks_to_write, value);
    }
}

#[cfg(test)]
mod test_delta {
    use super::*;

    //#[test]
    /// Test that we encode and decode low bits properly.
    fn test_delta_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            bs.write_delta(i);
        }
        println!("{:64b}", bs.data[0]);
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_delta());
        }
    }

    //#[test]
    /// Test that we encode and decode low bits properly.
    fn test_delta_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            bs.write_delta(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_delta());
        }
    }
}