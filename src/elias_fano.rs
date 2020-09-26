use super::*;
use fid::{BitVector, FID};
use std::mem::size_of;

pub struct EliasFano {
    n_of_elements: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
    high_bits: BitVector,
    low_bits: Vec<u64>,
}

impl EliasFano {
    /// Create a new elias-fano from a vector of sorted values.
    /// 
    /// # Arguments
    /// 
    /// * values: Vec<u64> - Vector of sorted integers to encode.
    /// * max: u64 - The maximum value within the vector.
    /// 
    pub fn new(values: Vec<u64>, max: u64) -> EliasFano {
        let low_bit_count = (max as f64 / values.len() as f64).log2().ceil() as u64;
        let low_bit_mask = (1 << low_bit_count) - 1;

        // add 2 to do the ceil and have brenchless primitives
        let low_size = 2
            + (values.len() as u64 * 8 * size_of::<u64>() as u64
                / (WORD_SIZE as u64 * low_bit_count));

        let mut result = EliasFano {
            low_bit_count,
            low_bit_mask,
            n_of_elements: values.len() as u64,
            high_bits: BitVector::new(),
            low_bits: vec![0; low_size as usize],
        };

        let mut last_high_value = 0;
        for (index, value) in values.iter().enumerate() {
            let high = value >> low_bit_count;

            for _ in last_high_value..high {
                result.high_bits.push(false);
            }

            result.high_bits.push(true);

            let low = value & ((1 << (low_bit_count + 1)) - 1);
            lowbit_write(&mut result.low_bits, index as u64, low, low_bit_count);

            last_high_value = high;
        }

        result
    }

    fn read_lowbits(&self, index: u64) -> u64 {
        lowbit_read(&self.low_bits, index, self.low_bit_count)
    }

    /// Return the rank of the value aka the number of elements <= than the value passed.
    pub fn rank(&self, value: u64) -> Option<u64> {
        let prefix = value >> self.low_bit_count;
        let suffix = value & self.low_bit_mask;
        let mut index = self.high_bits.rank1(prefix);
        while self.high_bits.get(index) {
            let curr_low_bits = self.read_lowbits(index);
            if curr_low_bits == suffix {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    /// Return the value of the chosen index. result = Eliasfano[index]
    pub fn select(&self, index: u64) -> Result<u64, String> {
        if index >= self.n_of_elements {
            return Err("Index out of bounds".to_string());
        }
        let bit_index = self.high_bits.select1(index);
        let low_bits = self.read_lowbits(bit_index);
        let high_bits = self.high_bits.rank0(bit_index);

        Ok((high_bits << self.low_bit_count) | low_bits)
    }
}
