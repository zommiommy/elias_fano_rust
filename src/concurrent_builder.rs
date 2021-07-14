use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
/// Builder that allows to concurrently build elias-fano.
/// The set method can be called by multiple threads.
/// Once the build is finished this struct can be converted to an EliasFano struct
/// by calling the `build` method which compute and store the indexes needed for
/// the constant time select (high-bits).
pub struct ConcurrentEliasFanoBuilder {
    high_bits: Vec<AtomicU64>,
    low_bits: Vec<AtomicU64>,
    number_of_inserted_elements: AtomicU64,
    max_index_found: AtomicU64,
    min_index_found: AtomicU64,
    number_of_elements: u64,
    universe: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
}

impl Default for ConcurrentEliasFanoBuilder {
    fn default() -> Self {
        ConcurrentEliasFanoBuilder{
            high_bits: Vec::new(),
            low_bits: Vec::new(),
            number_of_inserted_elements: AtomicU64::new(0),
            max_index_found: AtomicU64::new(0),
            min_index_found: AtomicU64::new(u64::MAX),
            number_of_elements: 0,
            universe: u64::MAX,
            low_bit_count: 0,
            low_bit_mask: 0,
        }
    }
}

impl ConcurrentEliasFanoBuilder {
    pub fn new(number_of_elements: u64, universe: u64) -> Result<ConcurrentEliasFanoBuilder, String> {
        // If the user says that there will be no elements, the builder will 
        // only use the high-bits to store the eventual values
        if number_of_elements == 0 {
            return Ok(ConcurrentEliasFanoBuilder{
                high_bits: Vec::new(),
                low_bits: Vec::new(),
                number_of_elements: 0,
                universe,
                ..Default::default()
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
        let low_size = get_vec_size(low_bit_count, number_of_elements as usize);
        // the number of bits will be at max the number of elements + max(high)
        // we need a ceil, but >> is floor so we add 1
        let high_size = ((number_of_elements + (universe >> low_bit_count)) as f64 / WORD_SIZE as f64).ceil() as u64;

        let high_bits: Vec<_> = (0..high_size).map(|_| AtomicU64::new(0)).collect();
        let low_bits : Vec<_> = (0..low_size ).map(|_| AtomicU64::new(0)).collect();

        Ok(ConcurrentEliasFanoBuilder {
            universe,
            low_bit_count,
            // Pre-rendered mask to execute a fast version of the mod operation.
            low_bit_mask: shr(0xffffffffffffffff, 64 - low_bit_count),
            high_bits,
            number_of_elements: number_of_elements as u64,
            low_bits,
            ..Default::default()
        })
    }

    /// Write the given value in the elias-fano, this method is
    /// safe from concurrency and allows to build elias-fano in parallel
    /// if the indices of the values are known in advance.
    pub fn set(&self, index: u64, value: u64) {
        let high = value >> self.low_bit_count;
        let low  = value & self.low_bit_mask;

        // write the low-bits
        concurrent_write(&self.low_bits, index, low, self.low_bit_count);

        // write the high-bits
        let idx = high + index;
        self.high_bits[(idx >> WORD_SHIFT) as usize].fetch_or(1 << (idx & WORD_MASK), Ordering::SeqCst);        

        self.number_of_inserted_elements.fetch_add(1, Ordering::SeqCst);
        self.max_index_found.fetch_max(index, Ordering::SeqCst);
        self.min_index_found.fetch_min(index, Ordering::SeqCst);
    }

    /// Return the **current** number of values inserted
    /// This is computed based on an atomic value with Sequentailly Consistent
    /// Ordering. If you want consistent values, it's your responsability to use
    /// it while no threads is writing.
    pub fn len(&self) -> usize {
        self.number_of_inserted_elements.load(Ordering::SeqCst) as usize
    }

    ///  Consume the builder and returns the built EliasFano struct.
    /// This step is not really parallel and will have to build the
    /// high-bits indices needed for the constant time select.
    pub fn build(self) -> Result<EliasFano, String> {
        // Remove the atomic type from the vector
        // this is not supposed to generate any instruction but it's meant to 
        // make the compiler happy.
        let (low_bits, high_bits) = unsafe { (
            std::mem::transmute::<Vec<_>, Vec<u64>>(self.low_bits),
            std::mem::transmute::<Vec<_>, Vec<u64>>(self.high_bits),
        )};

        if self.min_index_found.load(Ordering::SeqCst) != 0 
        {
            return Err(format!(
                concat!(
                    "The given indices were either not dense or ",
                    "they might have contained duplicates!\n",
                    "Specifically, the minimum index was expected to be zero, but was {}."
                ),
                self.min_index_found.load(Ordering::SeqCst)
            ));
        }
        
        if self.max_index_found.load(Ordering::SeqCst) != self.number_of_inserted_elements.load(Ordering::SeqCst).saturating_sub(1)
        {
            return Err(format!(
                concat!(
                    "The given indices were either not dense or ",
                    "they might have contained duplicates!\n",
                    "Specifically, the maximum index was expected to be equal to the number of elements in ",
                    "the set, which is {}, minus one, but was {}."
                ),
                self.number_of_inserted_elements.load(Ordering::SeqCst),
                self.max_index_found.load(Ordering::SeqCst),
            ));
        }

        let mut result = EliasFano {
            low_bits,
            high_bits: SimpleSelect::from_vec(high_bits),
            universe: self.universe,
            number_of_elements: self.number_of_elements,
            low_bit_count: self.low_bit_count,
            low_bit_mask: self.low_bit_mask,

            // We assume that this is correct, we could check this but it would 
            // mean that every thread should update it, thus possibily creating
            // a concurrency bottleneck.
            current_number_of_elements: self.number_of_elements,
            
            // These values are used to be able to push new values in the future.
            // We will initialize them to garbage, and then use the rank and select
            // methods to compute the right values
            last_high_value:0,
            last_value:0,
            last_index: self.number_of_elements,
        };

        if self.number_of_elements > 0 {
            let max_value = result.select(self.number_of_elements.saturating_sub(1)).unwrap();
            result.last_value = max_value;
            result.last_high_value = max_value >> self.low_bit_count;
        }

        Ok(result)
    }
}
