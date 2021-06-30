use super::*;

#[derive(Clone, Debug)]
/// Structure with index inspired by Vigna's simple select
pub struct HighBits {
    high_bits: Vec<u64>,
    high_bits_index_zeros: Vec<u64>,
    high_bits_index_ones: Vec<u64>,
    number_of_ones: u64,
    number_of_zeros: u64,
    len: u64,

    /// This is used to "buffer" the last word during the building
    /// and it might be useful to re-use to store the last seeked value to 
    /// speed up sequential accesses (i.e. iter in range)
    last_block: u64,
}

impl PartialEq for HighBits {
    fn eq(&self, other: &HighBits) -> bool {
        self.high_bits == other.high_bits
    }
}

impl HighBits {
    /// Allocate an empty high-bits structure
    pub fn new() -> HighBits {
        HighBits{
            high_bits: Vec::new(),
            high_bits_index_zeros: Vec::new(),
            high_bits_index_ones: Vec::new(),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
            last_block: 0,
        }
    }

    /// Allocate the high-bits with the right size for optimal speed
    pub fn with_capacity(capacity: usize) -> HighBits {
        HighBits{
            high_bits: Vec::with_capacity(capacity >> WORD_SHIFT),
            high_bits_index_zeros: Vec::with_capacity(capacity >> INDEX_SHIFT),
            high_bits_index_ones: Vec::with_capacity(capacity >> INDEX_SHIFT),
            number_of_ones: 0,
            number_of_zeros: 0,
            len: 0,
            last_block: 0,
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

        if self.len & WORD_MASK == 0 {
            self.high_bits.push(0);
        }


        self.len += 1;
    }

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
        code >>= in_word_reminder;

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

        // hint the compiler that this value will always be in [0, 64)
        // this should yield better code-gen, TODO! check the actual speed up
        unsafe{
            hint_in_range!(0..64, reminder_to_scan);
        }
        // scan the current word
        for i in 0..reminder_to_scan {
            // reset the lowest set bits (BLSR)
            code &= code - 1;
        }

        block_id + code.leading_zeros() as u64
    }



    pub fn select0(&self, index: u64) -> u64 {

        // scan the current word
        //while rank < index {
            // set the lowest unset bits (BLCS)
        //    code |= code + 1;
        //}
        0
    }


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