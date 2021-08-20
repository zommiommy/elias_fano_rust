use super::*;

impl<'a> IntoIterator for &'a SimpleSelect {
    type Item = u64;
    type IntoIter = SimpleSelectIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleSelectIterator::new(self)
    }
}

impl<'a> SimpleSelect {
    /// Return an iterator over all the indices of the bits set to one
    /// which are inside the provided range.
    pub fn iter_in_range(&'a self, range: Range<u64>) -> SimpleSelectIterator<'a> {
        SimpleSelectIterator::new_in_range(self, range)
    }
    
    /// return an Iterator over the indices of the bits set to one in the SimpleSelect.
    pub fn iter(&'a self) -> SimpleSelectIterator<'a> {
        self.into_iter()
    }
}

#[derive(Debug)]
pub struct SimpleSelectIterator<'a> {
    /// reference to the SimpleSelect which is being iter
    /// this is needed to get the reference to the high-bits
    father: &'a SimpleSelect,
    /// The current code already decoded
    current_code: u64,
    /// Current word index
    index: usize,
    /// Maximum index of where to stop
    max_index: usize,
    /// Maximum value the iter will return
    max: Option<u64>,
}


impl<'a> SimpleSelectIterator<'a> {

    /// Create a structure that iter over all the indices of the bits set to one
    /// which are inside the provided range.
    /// 
    /// This iterator should give the same result of:
    /// ```
    /// r.iter().filter(|x| range.contains(&x))
    /// ```
    #[inline]
    pub fn new_in_range(father: &SimpleSelect, range: Range<u64>) -> SimpleSelectIterator {
        if range.start >= father.len() {
            return SimpleSelectIterator{
                father:father,
                current_code: 0,
                index: 0,
                max_index: 0,
                max: None,
            };    
        }

        let block_id = range.start >> WORD_SHIFT;
        let in_word_reminder = range.start & WORD_MASK;
        let mut code = father.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code &= u64::MAX << in_word_reminder;

        SimpleSelectIterator{
            father:father,
            current_code: code,
            index: block_id as usize,
            max_index: father.high_bits.len(),
            max: Some(range.end),
        }
        
    }
    
    /// Create a structure that iter over all the indices of the bits set to one.
    #[inline]
    pub fn new(father: &SimpleSelect) -> SimpleSelectIterator {
        SimpleSelectIterator{
            father:father,
            current_code: *father.high_bits.get(0).unwrap_or(&0),
            index: 0,
            max_index: father.high_bits.len(),
            max: None,
        }
    }
}

impl<'a> Iterator for SimpleSelectIterator<'a> {
    type Item = u64;
    /// The iteration code takes inspiration from https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_code == 0 {
            self.index += 1;
            // if we are over just end the iterator
            if self.index >= self.max_index {
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
        let result = (self.index as u64 * WORD_SIZE) + t as u64;

        // Check if we exceeds the max value
        if let Some(_max) = &self.max {
            if result >= *_max {
                return None;
            }
        }

        Some(result)
    }
}