use super::*;
use std::ops::Range;

#[derive(Clone, Debug)]
/// Structure with index inspired by Vigna's simple select
pub struct SimpleSelect {
    high_bits: Vec<u64>,
    high_bits_index_zeros: Vec<u64>,
    high_bits_index_ones: Vec<u64>,
    number_of_ones: u64,
    number_of_zeros: u64,
    len: u64,
}

impl SimpleSelect {
    /// Return the memory used in bytes
    /// This is an approximation which considers 3 words extra for metadata for each
    /// vector
    pub fn size(&self) -> usize {
        use std::mem::size_of;
        3 * size_of::<u64>() + 
        (3  + self.high_bits.capacity()) * size_of::<u64>() +
        (3 + self.high_bits_index_zeros.capacity()) * size_of::<u64>() +
        (3 + self.high_bits_index_ones.capacity()) * size_of::<u64>()
    }

    /// Reduces the memory allocated to the minimum needed.
    pub fn shrink_to_fit(&mut self) {
        self.high_bits.shrink_to_fit();
        self.high_bits_index_zeros.shrink_to_fit();
        self.high_bits_index_ones.shrink_to_fit();
    }
}

impl PartialEq for SimpleSelect {
    fn eq(&self, other: &SimpleSelect) -> bool {
        // if needed this can be sped up by comparing the metadata before the vec
        self.high_bits == other.high_bits
    }
}

/// # Constructors
impl SimpleSelect {
    /// Allocate an empty high-bits structure
    pub fn new() -> SimpleSelect {
        SimpleSelect{
            high_bits: Vec::new(),
            high_bits_index_zeros: Vec::new(),
            high_bits_index_ones: Vec::new(),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
        }
    }

    /// Allocate the high-bits with the right size for optimal speed
    pub fn with_capacity(capacity: usize) -> SimpleSelect {
        SimpleSelect{
            high_bits: Vec::with_capacity(capacity >> WORD_SHIFT),
            high_bits_index_zeros: Vec::with_capacity(capacity >> INDEX_SHIFT),
            high_bits_index_ones: Vec::with_capacity(capacity >> INDEX_SHIFT),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
        }
    }

    /// Add the given bit to the end of the high-bits
    pub fn push(&mut self, value: bool) {
        if value {
            if self.number_of_ones & INDEX_MASK == 0 {
                self.high_bits_index_ones.push(self.len);
            }
            self.number_of_ones += 1;
        } else {
            if self.number_of_zeros & INDEX_MASK == 0 {
                self.high_bits_index_zeros.push(self.len);
            }
            self.number_of_zeros += 1;
        }

        if self.len & WORD_MASK == 0{
            self.high_bits.push(0);
        }

        if value {
            let last_idx = self.high_bits.len() - 1;
            let mut code = self.high_bits[last_idx];
            code |= 1 << (self.len & WORD_MASK);
            self.high_bits[last_idx] = code;
        }

        self.len += 1;
    }
}

/// # Getters
impl SimpleSelect {
    pub fn count_zeros(&self) -> u64 {
        self.number_of_zeros
    }

    pub fn count_ones(&self) -> u64 {
        self.number_of_ones
    }

    pub fn len(&self) -> u64 {
        self.len
    }
}


/// # Core functionalities
impl SimpleSelect {
    /// Returns the value of the bit of position `index`.
    pub fn get(&self, index: u64) -> bool {
        let word_idx = index >> WORD_SHIFT;
        let bit_idx = index & WORD_MASK;
        let bit_value = (self.high_bits[word_idx as usize] >> bit_idx) & 1;
        bit_value == 1
    }

    /// Returns the position of the `index`-th bit set to one.
    pub fn select1(&self, index: u64) -> u64 {
        // use the index to find in which block the value is
        let mut reminder_to_scan = index & INDEX_MASK;
        let index_idx = index >> INDEX_SHIFT;
        // the bit position of the biggest multiple of INDEX_SIZE which is
        // smaller than the choosen index, this is were we will start our search
        let bit_pos = self.high_bits_index_ones[index_idx as usize];

        // find in which word the start value is
        let mut block_id = bit_pos >> WORD_SHIFT;
        let in_word_reminder = bit_pos & WORD_MASK;

        // build the standard word to start scanning
        let mut code = self.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code &= u64::MAX << in_word_reminder;

        // use popcnt to find the right word
        loop {
            let popcnt = code.count_ones() as u64;
            if popcnt > reminder_to_scan {
                break
            } 
            block_id += 1;
            reminder_to_scan -= popcnt;
            code = self.high_bits[block_id as usize];
        }

        // scan the current word
        for _ in 0..reminder_to_scan {
            // reset the lowest set bits (BLSR)
            code &= code - 1;
        }

        (block_id * WORD_SIZE) + code.trailing_zeros() as u64
    }



    pub fn select0(&self, index: u64) -> u64 {
        // use the index to find in which block the value is
        let mut reminder_to_scan = index & INDEX_MASK;
        let index_idx = index >> INDEX_SHIFT;
        // the bit position of the biggest multiple of INDEX_SIZE which is
        // smaller than the choosen index, this is were we will start our search
        let bit_pos = self.high_bits_index_zeros[index_idx as usize];

        // find in which word the start value is
        let mut block_id = bit_pos >> WORD_SHIFT;
        let in_word_reminder = bit_pos & WORD_MASK;

        // build the standard word to start scanning
        let mut code = self.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code |= (1 << in_word_reminder) - 1;

        // use popcnt to find the right word
        loop {
            let popcnt = code.count_zeros() as u64;
            if popcnt > reminder_to_scan {
                break
            } 
            block_id += 1;
            reminder_to_scan -= popcnt;
            code = self.high_bits[block_id as usize];
        }

        // scan the current word
        for _ in 0..reminder_to_scan {
            // set the lowest set bits (BLCS)
            // saddly the BLCS instruction is nolonger
            // supported in any modern CPU, so select0 
            // will be slightly slower than select1
            code |= code + 1;
        }

        (block_id * WORD_SIZE) + code.trailing_ones() as u64
    }
}

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
}
impl<'a> SimpleSelect {
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

        let index_idx = range.start >> INDEX_SHIFT;
        let bit_pos = father.high_bits_index_ones[index_idx as usize];
        let block_id = bit_pos >> WORD_SHIFT;
        let in_word_reminder = bit_pos & WORD_MASK;
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
            current_code: father.high_bits[0],
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
            // if its the last block just dump it
            if self.index == self.max_index {
                return None;
            }
            // if we are over just end the iterator
            if self.index > self.max_index {
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
        let result = self.index as u64 * WORD_SIZE + t as u64;

        // Check if we exceeds the max value
        if let Some(_max) = &self.max {
            if result >= *_max {
                return None;
            }
        }

        Some(result)
    }
}

