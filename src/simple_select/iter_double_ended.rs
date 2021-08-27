use super::*;
use std::intrinsics::unlikely;

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
            .field("start_code", &format!("{:064b}", self.start_code))
            .field("start_index", &self.start_index)
            .field("end_code", &format!("{:064b}", self.end_code))
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

    pub fn new_in_range(father: &'a SimpleSelect, range: Range<u64>) -> SimpleSelectDobuleEndedIterator<'a> {
        if range.start >= father.len() {
            return SimpleSelectDobuleEndedIterator{
                start_code: 0,
                start_index: 0,
                end_code: 0,
                end_index: 0,
                len: 0,
                father, 
            };
        }
        // if the range starts and ends inside the same word of memory we need
        // special logic to clear the bits and avoid duplication of values
        if (range.start >> WORD_SHIFT) == (range.end >> WORD_SHIFT) {
            let idx = (range.start >> WORD_SHIFT) as usize;
            let mut code = father.high_bits[idx];

            // clean the higher and lwoer bits according to the range values
            code &= u64::MAX << (range.start & WORD_MASK);
            code &= !(!0_u64 << (64 - range.start & WORD_MASK));
            
            return SimpleSelectDobuleEndedIterator{
                len: code.count_ones() as usize,
                start_code: code,
                start_index: idx,
                end_code: 0,
                end_index: idx,
                father, 
            };
        }
        
        // general well behaved case
        let start_index = range.start >> WORD_SHIFT;
        let start_in_word_reminder = range.start & WORD_MASK;
        let mut start_code = father.high_bits[start_index as usize];
        start_code &= u64::MAX << start_in_word_reminder;

        let end_index = range.end >> WORD_SHIFT;
        let end_in_word_reminder = range.end & WORD_MASK;
        let mut end_code = father.high_bits[end_index as usize];
        end_code &= !(!0_u64 << end_in_word_reminder);

        SimpleSelectDobuleEndedIterator{
            start_code,
            start_index: start_index as usize,
            end_code,
            end_index: end_index as usize,
            len: (
                father.rank1(range.end) - father.rank1(range.start)
            ) as usize,
            father, 
        }
    }
}


impl<'a> Iterator for SimpleSelectDobuleEndedIterator<'a> {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.start_code == 0 {
            let tmp_idx = self.start_index + 1;
            if tmp_idx >= self.end_index {
                if self.end_code == 0 {
                    return None;
                }
                
                // get the index of the first one (we are guaranteed to have
                // at least one bit set to 1)
                let t = self.end_code.trailing_zeros();

                // clear it from the current code
                self.end_code &= self.end_code - 1;

                // compute the result value
                let result = (tmp_idx as u64 * WORD_SIZE) + t as u64;
                self.len = self.len.checked_sub(1).expect(&format!("SUb with overflow: {:#4?}", self));
                return Some(result);
            }

            self.start_index = tmp_idx;
            self.start_code = self.father.high_bits[self.start_index]
        }

        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = self.start_code.trailing_zeros();

        // clear it from the current code
        self.start_code &= self.start_code - 1;

        // compute the result value
        let result = (self.start_index as u64 * WORD_SIZE) + t as u64;
        self.len = self.len.checked_sub(1).expect(&format!("SUb with overflow: {:#4?}", self));
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a> ExactSizeIterator for SimpleSelectDobuleEndedIterator<'a> {}

impl<'a> DoubleEndedIterator for SimpleSelectDobuleEndedIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.end_code == 0 {
            let tmp_idx = self.end_index.saturating_sub(1);
            // if we reach the index of the start, we should finish the word
            // on which the other iter is already working
            if self.start_index >= tmp_idx {
                if self.start_code == 0 {
                    return None;
                }
                        
                // get the index of the last one (we are guaranteed to have
                // at least one bit set to 1)
                let t = 63 - self.start_code.leading_zeros();
                
                // clear it from the current code
                self.start_code ^= 1 << t;

                // compute the result value
                let result = (tmp_idx as u64 * WORD_SIZE) + t as u64;
                self.len = self.len.checked_sub(1).expect(&format!("SuB with overflow: {:#4?}", self));
                return Some(result);
            }

            // iter over the highbits
            self.end_index = tmp_idx;
            self.end_code = self.father.high_bits[self.end_index]
        }

        // get the index of the last one (we are guaranteed to have
        // at least one bit set to 1)
        let t = 63 - self.end_code.leading_zeros();
        
        // clear it from the current code
        self.end_code ^= 1 << t;

        // compute the result value
        let result = (self.end_index as u64 * WORD_SIZE) + t as u64;
        self.len = self.len.checked_sub(1).expect(&format!("SuB with overflow: {:#4?}", self));
        Some(result)
    }
}