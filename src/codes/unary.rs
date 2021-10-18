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
        // TODO!: check if simplifiable
        // Update the reminder
        self.bit_index  += (value & 63) as usize; 
        let reminder = self.bit_index >> 6;
        self.bit_index  &= 63;
        self.word_index += (value >> 6) as usize + reminder;

        // Write the bit
        self.data.resize(self.word_index + 1, 0);
        self.data[self.word_index] |= 1 << self.bit_index;
        self.bit_index += 1;
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
            bs.write_unary(i);
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
            bs.write_unary(i);
        }
        bs.seek(0);
        for i in (0..10_000).rev() {
            assert_eq!(i, bs.read_unary());
        }
    }
}