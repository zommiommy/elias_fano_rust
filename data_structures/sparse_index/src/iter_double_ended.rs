use super::*;
use core::intrinsics::unlikely;
use core::ops::Range;

impl<'a, const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {
    /// return an Iterator over the indices of the bits set to one in the SparseIndex.
    pub fn iter_double_ended(&'a self) -> SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
        SparseIndexDobuleEndedIterator::new(self)
    }

    /// return an Iterator over the indices of the bits set to one in the SparseIndex.
    pub fn iter_in_range_double_ended(
        &'a self,
        range: Range<usize>,
    ) -> SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
        SparseIndexDobuleEndedIterator::new_in_range(self, range)
    }
}

/// An iterator over the simple select ones
/// that can be itered in both directions and has a known length
pub struct SparseIndexDobuleEndedIterator<'a, const QUANTUM_LOG2: usize> {
    /// reference to the SparseIndex which is being iter
    /// this is needed to get the reference to the high-bits
    pub(crate) father: &'a SparseIndex<QUANTUM_LOG2>,

    pub(crate) start_code: usize,
    pub(crate) start_index: usize,
    pub(crate) end_index: usize,
    pub(crate) end_code: usize,
    pub(crate) len: usize,
}

impl<'a, const QUANTUM_LOG2: usize> core::fmt::Debug
    for SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SparseIndexDobuleEndedIterator")
            .field("start_code", &format!("{:064b}", self.start_code))
            .field("start_index", &self.start_index)
            .field("end_code", &format!("{:064b}", self.end_code))
            .field("end_index", &self.end_index)
            .field("len", &self.len)
            .finish()
    }
}

impl<'a, const QUANTUM_LOG2: usize> SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
    /// Create a new double-ended iterator for a given sparse index
    pub fn new(
        father: &'a SparseIndex<QUANTUM_LOG2>,
    ) -> SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
        SparseIndexDobuleEndedIterator {
            start_code: *father.high_bits.get(0).unwrap_or(&0),
            start_index: 0,
            end_code: *father.high_bits.last().unwrap_or(&0),
            end_index: father.high_bits.len().saturating_sub(1),
            len: father.count_ones() as _,
            father,
        }
    }

    /// Create a new double-ended iterator for a given sparse index optimized
    /// to only return the values in the given range
    pub fn new_in_range(
        father: &'a SparseIndex<QUANTUM_LOG2>,
        range: Range<usize>,
    ) -> SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
        if range.start >= father.len() {
            return SparseIndexDobuleEndedIterator {
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
            code &= usize::MAX << (range.start & WORD_BIT_SIZE_MASK);
            code &= !(!0_usize << (WORD_BIT_SIZE - range.start & WORD_BIT_SIZE_MASK));

            return SparseIndexDobuleEndedIterator {
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
        let start_in_word_reminder = range.start & WORD_BIT_SIZE_MASK;
        let mut start_code = father.high_bits[start_index as usize];
        start_code &= usize::MAX << start_in_word_reminder;

        let end_index = range.end >> WORD_SHIFT;
        let end_in_word_reminder = range.end & WORD_BIT_SIZE_MASK;
        let mut end_code = father.high_bits[end_index as usize];
        end_code &= !(!0_usize << end_in_word_reminder);

        SparseIndexDobuleEndedIterator {
            start_code,
            start_index: start_index as usize,
            end_code,
            end_index: end_index as usize,
            len: (father.rank1(range.end) - father.rank1(range.start)) as usize,
            father,
        }
    }
}

impl<'a, const QUANTUM_LOG2: usize> Iterator for SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while unlikely(self.start_code == 0) {
            let tmp_idx = self.start_index + 1;
            if unlikely(tmp_idx >= self.end_index) {
                if unlikely(self.end_code == 0) {
                    return None;
                }

                // get the index of the first one (we are guaranteed to have
                // at least one bit set to 1)
                let t = self.end_code.trailing_zeros();

                // clear it from the current code
                self.end_code &= self.end_code - 1;

                // compute the result value
                let result = (tmp_idx as usize * WORD_BIT_SIZE) + t as usize;
                self.len -= 1;
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
        let result = (self.start_index as usize * WORD_BIT_SIZE) + t as usize;
        self.len -= 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, const QUANTUM_LOG2: usize> ExactSizeIterator
    for SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2>
{
}

impl<'a, const QUANTUM_LOG2: usize> DoubleEndedIterator
    for SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while unlikely(self.end_code == 0) {
            let tmp_idx = self.end_index.saturating_sub(1);
            // if we reach the index of the start, we should finish the word
            // on which the other iter is already working
            if unlikely(self.start_index >= tmp_idx) {
                if unlikely(self.start_code == 0) {
                    return None;
                }

                // get the index of the last one (we are guaranteed to have
                // at least one bit set to 1)
                let t = (WORD_BIT_SIZE - 1) - self.start_code.leading_zeros() as usize;

                // clear it from the current code
                self.start_code ^= 1 << t;

                // compute the result value
                let result = (tmp_idx as usize * WORD_BIT_SIZE) + t as usize;
                self.len -= 1;
                return Some(result);
            }

            // iter over the highbits
            self.end_index = tmp_idx;
            self.end_code = self.father.high_bits[self.end_index]
        }

        // get the index of the last one (we are guaranteed to have
        // at least one bit set to 1)
        let t = (WORD_BIT_SIZE - 1) - self.end_code.leading_zeros() as usize;

        // clear it from the current code
        self.end_code ^= 1 << t;

        // compute the result value
        let result = (self.end_index as usize * WORD_BIT_SIZE) + t as usize;
        self.len -= 1;
        Some(result)
    }
}
