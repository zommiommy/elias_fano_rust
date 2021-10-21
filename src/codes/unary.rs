use super::BitStream;
use core::intrinsics::unlikely;

/// Optimal for gemetric distribution of ratio 1/2
impl BitStream {
    #[inline]
    pub fn read_unary(&mut self) -> u64 {
        let mut res = 0;
        loop {
            let word = self.data[self.word_index] >> self.bit_index;
            let x = word.trailing_zeros() as u64;
            // if the code is not finished, continue to the next word
            let bound = (64 - self.bit_index) as u64;
            if unlikely(x >= bound) {
                self.word_index += 1;
                self.bit_index = 0;
                res += bound;
                continue
            }

            // the code finish here
            self.skip(1 + x as usize);
            return x + res;
        }
    }

    #[inline]
    pub fn write_unary(&mut self, value: u64) {
        // Update the reminder
        let idx = (value + self.tell() as u64) as usize;

        let bit_index  = idx & 63; 
        let word_index = idx >> 6;

        self.data.resize(word_index + 2, 0);

        // Write the bit
        self.data[word_index] |= 1 << bit_index;
        
        self.seek(idx + 1);
    }

    pub fn size_unary(&mut self, value: u64) -> u64 {
        value + 1
    }
}

#[cfg(test)]
mod test_unary {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_unary_forward() {
        let mut bs = BitStream::new();
        for i in 0..10_000 {
            let idx = bs.tell();
            bs.write_unary(i);
            assert_eq!(bs.tell(), idx + bs.size_unary(i) as usize);
        }
        bs.seek(0);
        for i in 0..10_000 {
            assert_eq!(i, bs.read_unary());
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_unary_backward() {
        let mut bs = BitStream::new();
        for i in (0..10_000).rev() {
            let idx = bs.tell();
            bs.write_unary(i);
            assert_eq!(bs.tell(), idx + bs.size_unary(i) as usize);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_unary());
        }
    }
}