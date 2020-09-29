use super::*;
use fid::{BitVector, FID};
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::mem;

#[derive(Clone)]
pub struct EliasFano {
    universe: u64,
    number_of_elements: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
    low_bits: Vec<u64>,
    high_bits: BitVector,
    last_high_value: u64,
    last_value: u64,
    last_index: u64,
    current_number_of_elements: usize,
}

impl EliasFano {
    pub fn new(universe: u64, number_of_elements: usize) -> EliasFano {
        // Compute the size of the low bits.
        let low_bit_count = if universe >= number_of_elements as u64 {
            (universe as f64 / number_of_elements as f64).log2().floor() as u64
        } else {
            0
        };

        // add 2 to do the ceil and have brenchless primitives.
        let low_size = get_vec_size(low_bit_count, number_of_elements);

        EliasFano {
            universe,
            low_bit_count,
            // Pre-rendered mask to execute a fast version of the mod operation.
            low_bit_mask: (1 << low_bit_count) - 1,
            number_of_elements: number_of_elements as u64,
            high_bits: BitVector::new(),
            low_bits: vec![0; low_size as usize],
            last_high_value: 0,
            last_value: 0,
            last_index: 0,
            current_number_of_elements: 0,
        }
    }

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
    pub fn from_iter(
        values: impl Iterator<Item = u64>,
        universe: u64,
        number_of_elements: usize,
    ) -> Result<EliasFano, String> {
        if number_of_elements == 0 {
            return Err("Cannot create an Elias Fano with 0 values.".to_string());
        }

        let mut result = EliasFano::new(universe, number_of_elements);

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
        if values.is_empty() {
            return Err("Cannot create an Elias Fano with 0 values.".to_string());
        }
        EliasFano::from_iter(
            values.iter().cloned(),
            *values.last().unwrap(),
            values.len(),
        )
    }

    fn extract_high_low_bits(&self, value: u64) -> (u64, u64) {
        // The following is an efficient mod operation
        // It is the equivalent of executing:
        //
        // let low = value % low_bit_count;
        //
        // but faster.
        //
        (value >> self.low_bit_count, value & self.low_bit_mask)
    }

    pub fn push(&mut self, value: u64) -> Result<(), String> {
        if self.last_value > value {
            return Err(format!(
                concat!(
                    "Cannot initialize from an unsorted set of values! ",
                    "Previous value was {} but given value is {}.",
                ),
                self.last_value, value
            ));
        }
        self.last_value = value;
        self.current_number_of_elements += 1;

        // split into high and low bits
        let (high, low) = self.extract_high_low_bits(value);

        // The following for loop and push
        // are used to encode in inverted unary code for the high bits
        // of the data structure.
        for _ in self.last_high_value..high {
            self.high_bits.push(false);
        }
        self.high_bits.push(true);

        unsafe_write(&mut self.low_bits, self.last_index, low, self.low_bit_count);

        self.last_high_value = high;
        self.last_index += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.current_number_of_elements
    }

    pub fn is_empty(&self) -> bool {
        self.current_number_of_elements == 0
    }

    fn build_low_high_bits(&mut self, values: impl Iterator<Item = u64>) -> Result<(), String> {
        values.map(move |value| self.push(value)).collect()
    }

    fn read_lowbits(&self, index: u64) -> u64 {
        unsafe_read(&self.low_bits, index, self.low_bit_count)
    }

    /// Return iterator for the values in elias fano.
    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.number_of_elements).map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    pub fn par_iter(&self) -> impl ParallelIterator<Item = u64> + '_ {
        (0..self.number_of_elements)
            .into_par_iter()
            .map(move |index| self.unchecked_select(index))
    }

    /// Return iterator for the values in elias fano.
    pub fn enumerate(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        (0..self.number_of_elements).map(move |index| (index, self.unchecked_select(index)))
    }

    /// Return iterator for the values in elias fano.
    pub fn par_enumerate(&self) -> impl ParallelIterator<Item = (u64, u64)> + '_ {
        (0..self.number_of_elements)
            .into_par_iter()
            .map(move |index| (index, self.unchecked_select(index)))
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
        if value > self.universe {
            return self.number_of_elements;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1,
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
        match index < self.number_of_elements {
            true => Ok(self.unchecked_select(index)),
            false => Err(format!(
                "Given index {} is out of bound on a collection with {} elements.",
                index, self.number_of_elements
            )),
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

    pub fn contains(&self, value: u64) -> bool {
        if value > self.universe {
            return false;
        }
        // split into high and low
        let (high, low) = self.extract_high_low_bits(value);
        let mut index = match high == 0 {
            true => 0,
            false => self.high_bits.select0(high - 1) + 1,
        };
        // get the first guess
        let mut ones = self.high_bits.rank1(index);
        // handle the case where
        while self.high_bits.get(index) && self.read_lowbits(ones) < low {
            ones += 1;
            index += 1;
        }

        self.high_bits.get(index) && self.read_lowbits(ones) == low
    }

    /// Return the number of **bits** used by the structure
    pub fn size(&self) -> u64 {
        mem::size_of::<u64>() as u64 * (3 + 2 + self.low_bits.len()) as u64
            + self.high_bits.size() as u64
    }

    pub fn debug(&self) {
        println!("------------ELIAS-FANO------------------");
        println!("\tuniverse: {}", self.universe);
        println!("\tnumber_of_elements: {}", self.number_of_elements);
        println!("\tlow_bit_count: {}", self.low_bit_count);
        println!("\tlow_bit_mask: {}", self.low_bit_mask);
        println!("---------------low-bits-----------------");
        for i in 0..self.number_of_elements {
            print!("{}, ", self.read_lowbits(i));
        }
        println!("\n--------------high-bits-----------------");
        for i in 0..self.high_bits.len() {
            print!("{}", self.high_bits.get(i) as u64);
        }
        println!("\n--------------values--------------------");
        for v in self.iter() {
            print!("{}, ", v);
        }
        println!("\n----------------END---------------------");
    }
}
