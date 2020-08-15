use super::*;

pub struct EliasFano {
    n_of_elements: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
    high_bits: Vec<u64>,
    low_bits: Vec<u64>
}

impl EliasFano {
    pub fn new(values: Vec<u64>, max: u64) -> EliasFano {
        let max_bits = (max as f64).log2().ceil() as u64;
        let low_bit_count = (max as f64 / values.len() as f64).log2().ceil() as u64;
        let high_bit_count = max_bits - low_bit_count;

        let low_bit_mask = (1 << low_bit_count) - 1;

        // add 2 to do the ceil and have brenchless primitives
        let high_size = 2 + (values.len() as u64 / (WORD_SIZE as u64 * high_bit_count));
        let low_size  = 2 + (values.len() as u64 / (WORD_SIZE as u64 * low_bit_count));
        
        let high_bits = vec![0; high_size as usize];
        let low_bits  = vec![0; low_size  as usize];

        let result  = EliasFano{
            n_of_elements: values.len() as u64,
            low_bit_count: low_bit_count,
            low_bit_mask: low_bit_mask,
            high_bits:high_bits,
            low_bits:low_bits,
        };

        for value in values {
            let high = value >> low_bit_count;
            let index = result.set_high_bit(high);
            
            let low = value & ((1 << low_bit_count + 1) - 1);
            lowbit_write(&mut result.low_bits, index, low, low_bit_count);
            
            
        }

        result
    }

    pub fn rank(&self, value: u64) -> u64 {
        let prefix = value >> self.low_bit_count;
        let suffix = value & self.low_bit_mask;
        let mut ones = 0; // index
        let mut zeros = 0; // value
        for i in 0..self.high_bits.len() {
            let word = self.high_bits[i];
            let ones_in_current_word = population_count(word);
            let zeros_in_current_word = WORD_SIZE - ones_in_current_word;

            if zeros + zeros_in_current_word >= prefix {
                let value_index = index_of_nth_zeros_in_word(word,prefix - zeros);
                let high_bits = zeros + value_index;

                let possible_ones = n_of_consecutive_ones(word, value_index);
                for i in 0..possible_ones {
                    let current_index = value_index + i;
                    let low_bits = lowbit_read(&self.low_bits, current_index, self.low_bit_count);

                    if low_bits == suffix {
                        // FOUND
                        return current_index;
                    }
                    if low_bits > suffix{
                        // NOT FOUND
                        return 0;
                    }
                }
                
                
                // NOT FOUND
                return 0;
            }

            ones  += ones_in_current_word;
            zeros += zeros_in_current_word;
        }
        // index > len(elias fano)
        self.n_of_elements
    }

    pub fn select(&self, index: u64) -> u64 {
        if index >= self.n_of_elements{
            panic!("Index out of bounds");
        }
        let mut ones = 0;
        let mut zeros = 0;
        for word in &self.high_bits {
            let ones_in_current_word = population_count(*word);
            let zeros_in_current_word = WORD_SIZE - ones_in_current_word;

            if ones + ones_in_current_word >= index {
                zeros += index_of_nth_one_in_word(*word, index - ones);
                let high_bits = zeros << self.low_bit_count;
                let low_bits = lowbit_read(&self.low_bits, index, self.low_bit_count);
                return high_bits | low_bits;
            }

            ones  += ones_in_current_word;
            zeros += zeros_in_current_word;
        }
        panic!("Unreachable");
    }
}