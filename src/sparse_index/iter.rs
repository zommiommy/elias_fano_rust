use super::*;
use core::intrinsics::unlikely;
use core::ops::Range;

impl<'a, const QUANTUM_LOG2: usize> IntoIterator for &'a SparseIndex<QUANTUM_LOG2> {
    type Item = usize;
    type IntoIter = SparseIndexIterator<'a, QUANTUM_LOG2>;

    fn into_iter(self) -> Self::IntoIter {
        SparseIndexIterator::new(self)
    }
}

impl<'a, const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {
    /// Return an iterator over all the indices of the bits set to one
    /// which are inside the provided range.
    pub fn iter_in_range(&'a self, range: Range<usize>) -> SparseIndexIterator<'a, QUANTUM_LOG2> {
        SparseIndexIterator::new_in_range(self, range)
    }
    
    /// return an Iterator over the indices of the bits set to one in the SparseIndex.
    pub fn iter(&'a self) -> SparseIndexIterator<'a, QUANTUM_LOG2> {
        self.into_iter()
    }
}

#[derive(Debug)]
/// Iterator over a SparseIndex
pub struct SparseIndexIterator<'a, const QUANTUM_LOG2: usize> {
    /// reference to the SparseIndex which is being iter
    /// this is needed to get the reference to the high-bits
    father: &'a SparseIndex<QUANTUM_LOG2>,
    /// The current code already decoded
    current_code: usize,
    /// Current word index
    index: usize,
    /// Maximum index of where to stop
    max_index: usize,
    /// Maximum value the iter will return
    max: Option<usize>,
}


impl<'a, const QUANTUM_LOG2: usize> SparseIndexIterator<'a, QUANTUM_LOG2> {

    /// Create a structure that iter over all the indices of the bits set to one
    /// which are inside the provided range.
    #[inline]
    pub fn new_in_range(father: &SparseIndex<QUANTUM_LOG2>, range: Range<usize>) -> SparseIndexIterator<QUANTUM_LOG2> {
        if unlikely(range.start >= father.len()) {
            return SparseIndexIterator{
                father:father,
                current_code: 0,
                index: 0,
                max_index: 0,
                max: None,
            };    
        }

        let block_id = range.start >> WORD_SHIFT;
        let in_word_reminder = range.start & WORD_BIT_SIZE_MASK;
        let mut code = father.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code &= usize::MAX << in_word_reminder;

        SparseIndexIterator{
            father:father,
            current_code: code,
            index: block_id as usize,
            max_index: father.high_bits.len(),
            max: Some(range.end),
        }
        
    }
    
    /// Create a structure that iter over all the indices of the bits set to one.
    #[inline]
    pub fn new(father: &SparseIndex<QUANTUM_LOG2>) -> SparseIndexIterator<QUANTUM_LOG2> {
        SparseIndexIterator{
            father:father,
            current_code: *father.high_bits.get(0).unwrap_or(&0),
            index: 0,
            max_index: father.high_bits.len(),
            max: None,
        }
    }
}

impl<'a, const QUANTUM_LOG2: usize> Iterator for SparseIndexIterator<'a, QUANTUM_LOG2> {
    type Item = usize;
    /// The iteration code takes inspiration from <https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/>
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while unlikely(self.current_code == 0) {
            self.index += 1;
            // if we are over just end the iterator
            if unlikely(self.index >= self.max_index) {
                return None;
            }
            self.current_code = self.father.high_bits[self.index];   
        }

        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = self.current_code.trailing_zeros();

        // clear it from the current code
        self.current_code &= self.current_code - 1;

        // compute the result value
        let result = (self.index as usize * WORD_BIT_SIZE_MASK) + t as usize;

        // Check if we exceeds the max value
        if let Some(_max) = &self.max {
            if unlikely(result >= *_max) {
                return None;
            }
        }

        Some(result)
    }
}