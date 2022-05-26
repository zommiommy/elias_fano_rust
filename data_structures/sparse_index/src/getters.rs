use super::*;

/// # Getters
impl<const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {
    #[inline]
    /// Return how many zeros are present in the SparseIndexBitmap
    pub fn count_zeros(&self) -> usize {
        self.number_of_zeros
    }

    #[inline]
    /// Return how many ones are present in the SparseIndexBitmap
    pub fn count_ones(&self) -> usize {
        self.number_of_ones
    }

    #[inline]
    /// Reutrn how long is the SparseIndexBitmap
    pub fn len(&self) -> usize {
        self.len
    }
}

/// # Core functionalities
impl<const QUANTUM_LOG2: usize> SparseIndex<QUANTUM_LOG2> {
    /// Returns the value of the bit of position `index`.
    pub fn get(&self, index: usize) -> bool {
        let word_idx = index >> WORD_SHIFT;
        let bit_idx = index & WORD_BIT_SIZE_MASK;
        let bit_value = (self.high_bits[word_idx as usize] >> bit_idx) & 1;
        bit_value == 1
    }

    /// Returns the position of the `index`-th bit set to one.
    pub fn select1(&self, index: usize) -> usize {
        // use the index to find in which block the value is
        let mut reminder_to_scan = index & power_of_two_to_mask(QUANTUM_LOG2);
        let index_idx = index >> QUANTUM_LOG2;
        // the bit position of the biggest multiple of INDEX_SIZE which is
        // smaller than the choosen index, this is were we will start our search
        let bit_pos = self.high_bits_index_ones[index_idx as usize];

        // find in which word the start value is
        let mut block_id = bit_pos >> WORD_SHIFT;
        let in_word_reminder = bit_pos & WORD_BIT_SIZE_MASK;

        // build the standard word to start scanning
        let mut code = self.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code &= usize::MAX << in_word_reminder;

        // use popcnt to find the right word
        loop {
            let popcnt = code.count_ones() as usize;
            if popcnt > reminder_to_scan {
                break;
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

        (block_id * WORD_BIT_SIZE) + code.trailing_zeros() as usize
    }

    /// Returns the position of the `index`-th bit set to zero.
    pub fn select0(&self, index: usize) -> usize {
        // use the index to find in which block the value is
        let mut reminder_to_scan = index & power_of_two_to_mask(QUANTUM_LOG2);
        let index_idx = index >> QUANTUM_LOG2;
        // the bit position of the biggest multiple of INDEX_SIZE which is
        // smaller than the choosen index, this is were we will start our search
        let bit_pos = self.high_bits_index_zeros[index_idx as usize];

        // find in which word the start value is
        let mut block_id = bit_pos >> WORD_SHIFT;
        let in_word_reminder = bit_pos & WORD_BIT_SIZE_MASK;

        // build the standard word to start scanning
        let mut code = self.high_bits[block_id as usize];

        // clean the "already parsed lower bits"
        code |= (1 << in_word_reminder) - 1;

        //////////////////////////////////////////////////////////////

        // use popcnt to find the right word
        loop {
            let popcnt = code.count_zeros() as usize;
            if popcnt > reminder_to_scan {
                break;
            }
            block_id += 1;
            reminder_to_scan -= popcnt;
            code = self.high_bits[block_id as usize];
        }

        // scan the current word
        // Example:
        // code: 01101010110011100011101110
        //                  ^ wanted zero
        // reminder_to_scan: 5
        // ->
        // code: 01101010110011111111111111
        //                  ^ wanted zero
        for _ in 0..reminder_to_scan {
            // set the lowest set bits (BLCS)
            // saddly the BLCS instruction is nolonger
            // supported in any modern CPU, so select0
            // will be slightly slower than select1
            code |= code + 1;
        }

        (block_id * WORD_BIT_SIZE) + code.trailing_ones() as usize
    }

    /// Return the number of bits set to one from the start to the given `index`
    /// so in the range [0, `index`).
    ///
    /// This is basically a select + a binary search so it should be a bit
    /// slower than a select.
    pub fn rank1(&self, index: usize) -> usize {
        if index >= self.len() {
            return self.count_ones();
        }
        if self.count_ones() == 0 {
            return 0;
        }
        // use the ones index to search for in which word the index fall
        match self.high_bits_index_ones.binary_search(&index) {
            // fast path, luckily the index is one found in the index
            // so we can directly compute the number of ones
            Ok(idx) => (idx as usize) << QUANTUM_LOG2,
            Err(idx) => {
                // Find the biggest index value smaller than the index
                let idx = idx.saturating_sub(1);
                let mut res = (idx as usize) << QUANTUM_LOG2;

                // Read the index to start at a better position for the count
                let bit_pos = self.high_bits_index_ones[idx];

                // find the word index and the bit index inside the word
                let mut current_idx = bit_pos >> WORD_SHIFT;
                let bits_to_ignore = bit_pos & WORD_BIT_SIZE_MASK;

                // setup the word of memory so that the already counted
                // bits are cleaned to avoid double counting
                let mut current_word = self.high_bits[current_idx as usize];
                current_word &= !0_usize << bits_to_ignore;

                // compute how many bits are left to be scanned
                let mut bits_left = (index + bits_to_ignore).saturating_sub(bit_pos);

                // We can quickly skip words by popcnt-ing them
                while bits_left >= WORD_BIT_SIZE {
                    res += current_word.count_ones() as usize;
                    bits_left -= WORD_BIT_SIZE;
                    current_idx += 1;
                    current_word = self.high_bits[current_idx as usize];
                }

                // count the ones in the last word.
                // we will clean out the bits we don't care about and popcnt
                res += (current_word & !(!0_usize << (bits_left as u32))).count_ones() as usize;

                res
            }
        }
    }

    /// Return the number of bits set to zero from the start to the given `index`
    /// so in the range [0, `index`).
    ///
    /// This is basically a select + a binary search so it should be a bit
    /// slower than a select.
    pub fn rank0(&self, index: usize) -> usize {
        if index >= self.len() {
            return self.count_zeros();
        }
        if self.count_zeros() == 0 {
            return 0;
        }
        // use the ones index to search for in which word the index fall
        match self.high_bits_index_zeros.binary_search(&index) {
            // fast path, luckily the index is one found in the index
            // so we can directly compute the number of ones
            Ok(idx) => (idx as usize) << QUANTUM_LOG2,
            Err(idx) => {
                // Find the biggest index value smaller than the index
                let idx = idx.saturating_sub(1);
                let mut res = (idx as usize) << QUANTUM_LOG2;

                // Read the index to start at a better position for the count
                let bit_pos = self.high_bits_index_zeros[idx];

                // find the word index and the bit index inside the word
                let mut current_idx = bit_pos >> WORD_SHIFT;
                let bits_to_ignore = bit_pos & WORD_BIT_SIZE_MASK;

                // setup the word of memory so that the already counted
                // bits are cleaned to avoid double counting
                let mut current_word = !self.high_bits[current_idx as usize];
                current_word &= !0_usize << bits_to_ignore;

                // compute how many bits are left to be scanned
                let mut bits_left = (index + bits_to_ignore).saturating_sub(bit_pos);

                // We can quickly skip words by popcnt-ing them
                while bits_left >= WORD_BIT_SIZE {
                    res += current_word.count_ones() as usize;
                    bits_left -= WORD_BIT_SIZE;
                    current_idx += 1;
                    current_word = !self.high_bits[current_idx as usize];
                }

                // count the ones in the last word.
                // we will clean out the bits we don't care about and popcnt
                res += (current_word & !(!0_usize << (bits_left as u32))).count_ones() as usize;

                res
            }
        }
    }
}
