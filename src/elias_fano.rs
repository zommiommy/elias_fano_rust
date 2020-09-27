use super::*;
use std::mem;
use fid::{BitVector, FID};
use std::ops::Index;

#[derive(Clone)]
pub struct EliasFano {
    universe: u64,
    n_of_elements: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
    low_bits: Vec<u64>,
    high_bits: BitVector,
}


impl EliasFano {
    /// Create a new elias-fano from an iterable of **sorted values**.
    /// 
    /// # Arguments
    /// 
    /// * values: &[u64] - Vector of sorted integers to encode.
    /// * max: u64 - The maximum value within the vector.
    /// ```
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_iter(vector.iter().cloned(), *vector.last().unwrap(), vector.len()).unwrap();
    /// ```
    pub fn from_iter(values: impl Iterator<Item = u64>, universe: u64, n_of_elements: usize) -> Result<EliasFano, String> {
        if n_of_elements == 0 {
            return Err("Cannot create an Elias Fano with 0 values.".to_string());
        }
        // Compute the size of the low bits.
        let low_bit_count = if universe >= n_of_elements as u64 {
            (universe as f64 / n_of_elements as f64).log2().floor() as u64 
        } else {
            0
        };

        // add 2 to do the ceil and have brenchless primitives.
        let low_size = get_vec_size(low_bit_count, n_of_elements);

        let mut result = EliasFano {
            universe,
            low_bit_count,
            // Pre-rendered mask to execute a fast version of the mod operation.
            low_bit_mask: (1 << low_bit_count) - 1,
            n_of_elements: n_of_elements as u64,
            high_bits: BitVector::new(),
            low_bits: vec![0; low_size as usize],
        };

        result.build_low_high_bits(values)?;

        Ok(result)
    }

    /// Create a new elias-fano from a vector of **sorted values**.
    /// 
    /// # Arguments
    /// 
    /// * values: &[u64] - Vector of sorted integers to encode.
    /// * max: u64 - The maximum value within the vector.
    /// 
    /// ```
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_vec(&vector).unwrap();
    /// ```
    pub fn from_vec(values: &[u64]) -> Result<EliasFano, String> {
        if values.len() == 0 {
            return Err("Cannot create an Elias Fano with 0 values.".to_string());
        }
        EliasFano::from_iter(values.iter().cloned(), *values.last().unwrap(), values.len())
    }

    fn extract_high_low_bits(&self, value:u64) -> (u64, u64) {
        // The following is an efficient mod operation
        // It is the equivalent of executing:
        //
        // let low = value % low_bit_count;
        //
        // but faster.
        //
        (value >> self.low_bit_count, value & self.low_bit_mask)
    }

    fn build_low_high_bits(&mut self, values: impl Iterator<Item = u64 > ) -> Result<(), String> {
        let mut last_high_value = 0;
        let mut last_value = 0;
        for (index, value) in values.enumerate() {
            // check that the values are actually sorted.
            if last_value > value {
                return Err(format!(concat!(
                    "Cannot initialize from an unsorted set of values!\n",
                    "At the index {} there is {} but the value before was {}."
                ),
                index, value, last_value
            ));
            }
            last_value = value;

            // split into high and low bits
            let (high, low) = self.extract_high_low_bits(value);

            // The following for loop and push
            // are used to encode in inverted unary code for the high bits
            // of the data structure.
            for _ in last_high_value..high {
                self.high_bits.push(false);
            }
            self.high_bits.push(true);

            
            lowbit_write(&mut self.low_bits, index as u64, low, self.low_bit_count);

            last_high_value = high;
        }
        Ok(())
    }

    fn read_lowbits(&self, index: u64) -> u64 {
        lowbit_read(&self.low_bits, index, self.low_bit_count)
    }

    /// Return the number of elements <= to the given value.
    /// If the element is in the set, this is equivalent to the 
    /// index of the first instance of the given value.
    /// 
    /// This means that if in the vector there are multiple equal values,
    /// the index returned will always be the one of the first.
    /// 
    /// # Arguments
    /// 
    /// * `value`: u64 - Value whose rank is to be extracted.
    /// 
    /// # Usage example
    /// 
    /// Let's see an example. If I have the vector:
    /// 
    /// ```rust
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_vec(&vector).unwrap();
    /// 
    /// assert_eq!(ef.rank(15), 3);
    /// assert_eq!(ef.rank(8), 1);
    /// assert_eq!(ef.rank(17), 4);
    /// ```
    /// 
    pub fn rank(&self, value: u64) -> u64 {
        if value  > self.universe {
            return self.n_of_elements;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1
        };
        // get the first guess
        let mut ones = self.high_bits.rank1(index);
        // handle the case where 
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }
        
        ones
    }

    /// Return the value of the chosen index.
    /// 
    /// This version of the select does a check to verify that the given index
    /// is not greater than the allowed amount. To avoid this check in conditions
    /// where the performance is key, consider using the method `unchecked_select`
    /// which raises a panic when the conditions are not met. 
    /// 
    /// # Arguments
    /// 
    /// * index: u64 - Index of the value to be extract.
    pub fn select(&self, index: u64) -> Result<u64, String> {
        match index < self.n_of_elements {
            true => Ok(self.unchecked_select(index)),
            false => Err(format!(
                "Given index {} is out of bound on a collection with {} elements.",
                index,
                self.n_of_elements
            ))
        }   
    }

    /// Return the value of the chosen index without executing checks.
    /// 
    /// # Arguments
    /// 
    /// * index: u64 - Index of the value to be extract.
    pub fn unchecked_select(&self, index: u64) -> u64 {
        let bit_index = self.high_bits.select1(index);
        let high_bits = self.high_bits.rank0(bit_index);
        let low_bits = self.read_lowbits(index);
        (high_bits << self.low_bit_count) | low_bits
    }

    /// Return the number of **bits** used by the structure
    pub fn size(&self) -> u64 {
        mem::size_of::<u64>() as u64 * (3 + 2 + self.low_bits.len()) as u64 + self.high_bits.size() as u64
    }

    pub fn debug(&self) {
        println!("------------ELIAS-FANO------------------");
        println!("\tuniverse: {}", self.universe);
        println!("\tn_of_elements: {}", self.n_of_elements);
        println!("\tlow_bit_count: {}", self.low_bit_count);
        println!("\tlow_bit_mask: {}", self.low_bit_mask);
        println!("---------------low-bits-----------------");
        for i in 0..self.n_of_elements {
            print!("{}, ", self.read_lowbits(i));
        }
        print!("\n");
        println!("--------------high-bits-----------------");
        for i in 0..self.high_bits.len() {
            print!("{}", self.high_bits.get(i) as u64);
        }
        print!("\n");
        println!("--------------values--------------------");
        for i in 0..self.n_of_elements {
            print!("{}, ", self.select(i).unwrap());
        }
        print!("\n");
        println!("----------------END---------------------");
    }
}