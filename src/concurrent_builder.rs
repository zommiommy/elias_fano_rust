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
    number_of_elements: u64,
    universe: u64,
    low_bit_count: u64,
    low_bit_mask: u64,
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
                universe: universe,
                low_bit_count: 0,
                low_bit_mask: 0,
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

        let high_bits = (0..(2 * number_of_elements) >> WORD_SHIFT).map(|_| AtomicU64::new(0)).collect();
        let low_bits = (0..low_size).map(|_| AtomicU64::new(0)).collect();

        Ok(ConcurrentEliasFanoBuilder {
            universe,
            low_bit_count,
            // Pre-rendered mask to execute a fast version of the mod operation.
            low_bit_mask: shr(0xffffffffffffffff, 64 - low_bit_count),
            high_bits,
            number_of_elements: number_of_elements as u64,
            low_bits,
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
    }

    ///  Consume the builder and returns the built EliasFano struct.
    /// This step is not really parallel and will have to build the
    /// high-bits indices needed for the constant time select.
    pub fn build(self) -> EliasFano {
        // Remove the atomic type from the vector
        // this is not supposed to generate any instruction but it's meant to 
        // make the compiler happy.
        let (low_bits, high_bits) = unsafe { (
            std::mem::transmute::<Vec<_>, Vec<u64>>(self.low_bits),
            std::mem::transmute::<Vec<_>, Vec<u64>>(self.high_bits),
        )};

        let result = EliasFano {
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
            last_index:0,
        };

        // TODO! actually fix those values

        result
    }
}