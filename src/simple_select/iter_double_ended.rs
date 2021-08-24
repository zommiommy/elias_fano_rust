use super::*;

/// An iterator over the simple select ones
/// that can be itered in both directions and has a known length
pub struct SimpleSelectDobuleEndedIterator<'a> {
    /// reference to the SimpleSelect which is being iter
    /// this is needed to get the reference to the high-bits
    father: &'a SimpleSelect,

    start_code: u64,
    start_index: usize,
    end_index: usize,
    end_code: u64,
    len: usize,
}

impl<'a> std::fmt::Debug for SimpleSelectDobuleEndedIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimpleSelectDobuleEndedIterator")
            .field("start_code", &self.start_code)
            .field("start_index", &self.start_index)
            .field("end_code", &self.end_code)
            .field("end_index", &self.end_index)
            .field("len", &self.len)
            .finish()
    }
}


impl<'a> SimpleSelectDobuleEndedIterator<'a> {
    pub fn new(father: &'a SimpleSelect) -> SimpleSelectDobuleEndedIterator<'a> {
        SimpleSelectDobuleEndedIterator{
            start_code: *father.high_bits.get(0).unwrap_or(&0),
            start_index: 0,
            end_code: *father.high_bits.last().unwrap_or(&0),
            end_index: father.high_bits.len().saturating_sub(1),
            len: father.count_ones() as _,
            father, 
        }
    }
}


impl<'a> Iterator for SimpleSelectDobuleEndedIterator<'a> {
    type Item = u64;
    /// The iteration code takes inspiration from https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut code = &mut self.start_code;
        while *code == 0 {
            if self.start_index == self.end_index {
                if self.end_code == 0 {
                    return None;
                }
                code = &mut self.end_code;
            } else {
                self.start_index += 1;
                *code = self.father.high_bits[self.start_index]
            }
        }

        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = code.trailing_zeros();

        // clear it from the current code
        *code &= *code - 1;

        // compute the result value
        let result = (self.start_index as u64 * WORD_SIZE) + t as u64;

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for SimpleSelectDobuleEndedIterator<'a> {}

impl<'a> DoubleEndedIterator for SimpleSelectDobuleEndedIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut code = &mut self.end_code;
        while *code == 0 {
            if self.start_index >= self.end_index {
                if self.start_code == 0 {
                    return None;
                }
                code = &mut self.start_code;
            } else {
                self.end_index = self.end_index.saturating_sub(1);
                *code = self.father.high_bits[self.end_index]
            }
        }

        // get the index of the last one (we are guaranteed to have
        // at least one bit set to 1)
        let t = 63 - code.leading_zeros();
        
        // clear it from the current code
        *code ^= 1 << t;

        // compute the result value
        let result = (self.end_index as u64 * WORD_SIZE) + t as u64;

        Some(result)
    }
}