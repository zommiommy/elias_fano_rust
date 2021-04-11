use super::*;
use rsdict::RsDict;

impl EliasFano {

    #[inline]
    pub fn new(universe: u64, number_of_elements: usize) -> Result<EliasFano, String> {
        if number_of_elements == 0 {
            return Ok(EliasFano{
                universe: universe,
                low_bit_count: 0,
                low_bit_mask:  0,
                number_of_elements: 0,
                high_bits: RsDict::new(),
                low_bits: vec![],
                last_high_value: 0,
                last_value: 0,
                last_index: 0,
                current_number_of_elements: 0,
            });
        }
        // Compute the size of the low bits.
        let low_bit_count = if universe >= number_of_elements as u64 {
            (universe as f64 / number_of_elements as f64).log2().floor() as u64
        } else {
            0
        };

        // saturate at the max we can handle
        if low_bit_count > 64 {
            return Err(format!(concat!(
                    "The lowbits are too big, we only support 64 bits for the low parts.",
                    "The value were universe {} number_of_elements {}"
                ),
                universe, number_of_elements
            ));
        }

        // add 2 to do the ceil and have brenchless primitives.
        let low_size = get_vec_size(low_bit_count, number_of_elements);

        Ok(EliasFano {
            universe,
            low_bit_count,
            // Pre-rendered mask to execute a fast version of the mod operation.
            low_bit_mask: shr(0xffffffffffffffff, (64 - low_bit_count) as usize),
            number_of_elements: number_of_elements as u64,
            high_bits: RsDict::new(),
            low_bits: vec![0; low_size as usize],
            last_high_value: 0,
            last_value: 0,
            last_index: 0,
            current_number_of_elements: 0,
        })
    }

    /// Create a new elias-fano from an iterable of **sorted values**.
    ///    low_bits: Vec<u64>,

    /// # Arguments
    ///
    /// * values: &[u64] - Vector of sorted integers to encode.
    /// * max: u64 - The maximum value within the vector.
    /// ```
    /// # use elias_fano_rust::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::from_iter(vector.iter().cloned(), *vector.last().unwrap(), vector.len()).unwrap();
    /// ```
    #[inline]
    pub fn from_iter(
        values: impl Iterator<Item = u64>,
        universe: u64,
        number_of_elements: usize,
    ) -> Result<EliasFano, String> {
        let mut result = EliasFano::new(universe, number_of_elements)?;

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
    #[inline]
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

    #[inline]
    pub fn unchecked_push(&mut self, value: u64) {
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

        #[cfg(not(feature = "unsafe"))]
        safe_write(&mut self.low_bits, self.last_index, low, self.low_bit_count);
        #[cfg(feature = "unsafe")]
        unsafe_write(&mut self.low_bits, self.last_index, low, self.low_bit_count);

        self.last_high_value = high;
        self.last_index += 1;
    }

    #[inline]
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
        if self.current_number_of_elements >= self.number_of_elements {
            return Err(format!(
                concat!(
                    "Cannot push anymore values inside of the Elias-Fano ",
                    "because it already reached the maximum number of elements ",
                    "that was passed during the initialization {}."
                ),
                self.number_of_elements
            ));
        }
        self.unchecked_push(value);
        Ok(())
    }
}
