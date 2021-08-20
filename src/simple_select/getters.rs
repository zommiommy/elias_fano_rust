use super::*;

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

    /// Returns the position of the `index`-th bit set to zero.
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

    /// Return the number of bits set to one from the start to the given `index` 
    /// so in the range [0, `index`).
    ///
    /// This is basically a select + a binary search so it should be a bit
    /// slower than a select.
    pub fn rank1(&self, index: u64) -> u64 {
        // use the ones index to search for in which word the index fall
        match self.high_bits_index_ones.binary_search(&index) {
            // fast path, luckily the index is one found in the index
            // so we can directly compute the number of ones
            Ok(idx) => {
                (idx as u64) << INDEX_SHIFT
            },
            Err(idx) => {
                // Find the biggest index value smaller than the index
                let idx = idx.saturating_sub(1);
                let mut res = (idx as u64) << INDEX_SHIFT;
                
                // Read the index to start at a better position for the count
                let bit_pos = self.high_bits_index_ones[idx];

                // find the word index and the bit index inside the word
                let mut current_idx = bit_pos >> WORD_SHIFT;
                let bits_to_ignore  = bit_pos & WORD_MASK;

                // setup the word of memory so that the already counted
                // bits are cleaned to avoid double counting
                let mut current_word = self.high_bits[current_idx as usize];
                current_word &= !0_u64 << bits_to_ignore;

                // compute how many bits are left to be scanned
                let mut bits_left = index + bits_to_ignore - bit_pos;
                
                // We can quickly skip words by popcnt-ing them
                while bits_left > 63 {
                    res += current_word.count_ones() as u64;
                    bits_left -= 64;
                    current_idx += 1;
                    current_word = self.high_bits[current_idx as usize];
                }

                // count the ones in the last word.
                // we will clean out the bits we don't care about and popcnt
                res += (
                        current_word 
                        & !(!0_u64 << (bits_left as u32))
                    ).count_ones() as u64;

                res
            }
        }
    }

    /// Return the number of bits set to zero from the start to the given `index` 
    /// so in the range [0, `index`).
    ///
    /// This is basically a select + a binary search so it should be a bit
    /// slower than a select.
    pub fn rank0(&self, index: u64) -> u64 {
        // use the ones index to search for in which word the index fall
        match self.high_bits_index_zeros.binary_search(&index) {
            // fast path, luckily the index is one found in the index
            // so we can directly compute the number of ones
            Ok(idx) => {
                (idx as u64) << INDEX_SHIFT
            },
            Err(idx) => {
                // Find the biggest index value smaller than the index
                let idx = idx.saturating_sub(1);
                let mut res = (idx as u64) << INDEX_SHIFT;
                
                // Read the index to start at a better position for the count
                let bit_pos = self.high_bits_index_zeros[idx];

                // find the word index and the bit index inside the word
                let mut current_idx = bit_pos >> WORD_SHIFT;
                let bits_to_ignore  = bit_pos & WORD_MASK;

                // setup the word of memory so that the already counted
                // bits are cleaned to avoid double counting
                let mut current_word = !self.high_bits[current_idx as usize];
                current_word &= !0_u64 << bits_to_ignore;

                // compute how many bits are left to be scanned
                let mut bits_left = index + bits_to_ignore - bit_pos;
                
                // We can quickly skip words by popcnt-ing them
                while bits_left > 63 {
                    res += current_word.count_ones() as u64;
                    bits_left -= 64;
                    current_idx += 1;
                    current_word = !self.high_bits[current_idx as usize];
                }

                // count the ones in the last word.
                // we will clean out the bits we don't care about and popcnt
                res += (
                        current_word 
                        & !(!0_u64 << (bits_left as u32))
                    ).count_ones() as u64;

                res
            }
        }
    }
    
}
