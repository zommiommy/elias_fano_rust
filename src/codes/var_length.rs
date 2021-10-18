use crate::utils::fast_log2_ceil;
use super::BitStream;

impl BitStream {

    #[inline]
    pub fn read_var_length<const K: u64>(&mut self) -> u64 {
        let len = self.read_unary();

        // read `len` blocks of `K` bits
        // on x86_64 this constant multiplication will be converted
        // to a LEA instruction which is MUCH faster than a MUL
        self.read_bits(K * len)
    }

    #[inline]
    pub fn write_var_length<const K: u64>(&mut self, value: u64) {
        // compute how many blocks of `K` bits we are going to use
        let number_of_blocks_to_write = (fast_log2_ceil(value + 1) as f64 / K as f64).ceil() as u64;
        self.write_unary(number_of_blocks_to_write);

        // write `prefix` blocks of `K` bits
        self.write_bits(K * number_of_blocks_to_write, value);
    }
}

#[cfg(test)]
mod test_var_length {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_var_length_forward() {
        let mut bs = BitStream::new();
        for i in 0..100 {
            bs.write_var_length::<4>(i);
        }
        println!("{:064b}", bs.data[0]);
        bs.seek(0);
        for i in 0..100 {
            assert_eq!(i, bs.read_var_length::<4>());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_var_length_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            bs.write_var_length::<3>(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_var_length::<3>());
        }
    }
}