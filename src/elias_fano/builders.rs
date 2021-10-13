use super::*;

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {

    #[inline]
    pub fn new(universe: u64, number_of_elements: usize) -> Result<EliasFano<QUANTUM_LOG2>, String> {
        if number_of_elements == 0 {
            return Ok(EliasFano{
                universe: universe,
                number_of_elements: 0,
                high_bits: SparseIndex::new(),
                low_bits: CompactArray::new(0),
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
        let low_bits= CompactArray::with_capacity(low_bit_count, number_of_elements);

        Ok(EliasFano {
            universe,
            // Pre-rendered mask to execute a fast version of the mod operation.
            high_bits: SparseIndex::with_capacity(2 * number_of_elements),
            number_of_elements: number_of_elements as u64,
            low_bits,
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
    /// # use elias_fano_rust::elias_fano::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::<10>::from_iter(vector.iter().cloned(), *vector.last().unwrap(), vector.len()).unwrap();
    /// ```
    #[inline]
    pub fn from_iter(
        values: impl Iterator<Item = u64>,
        universe: u64,
        number_of_elements: usize,
    ) -> Result<EliasFano<QUANTUM_LOG2>, String> {
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
    /// # use elias_fano_rust::elias_fano::EliasFano;
    /// let vector = [5, 8, 8, 15, 32];
    /// let ef = EliasFano::<10>::from_vec(&vector).unwrap();
    /// ```
    #[inline]
    pub fn from_vec(values: &[u64]) -> Result<EliasFano<QUANTUM_LOG2>, String> {
        EliasFano::from_iter(
            values.iter().cloned(),
            *values.last().unwrap_or(&0),
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
        
        self.low_bits.write(self.last_index, low);

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
