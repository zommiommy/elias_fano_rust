use super::*;
use crate::{constants::WORD_BIT_SIZE, sparse_index::SparseIndexConcurrentBuilder};
use alloc::string::String;

#[derive(Debug)]
/// Builder that allows to concurrently build elias-fano.
/// The set method can be called by multiple threads.
/// Once the build is finished this struct can be converted to an EliasFano struct
/// by calling the `build` method which compute and store the indexes needed for
/// the constant time select (high-bits).
pub struct ConcurrentEliasFanoBuilder<const QUANTUM_LOG2: usize> {
    high_bits: SparseIndexConcurrentBuilder<QUANTUM_LOG2>,
    low_bits: CompactArray,
    number_of_elements: usize,
    universe: usize,
}
impl<const QUANTUM_LOG2: usize> Default for ConcurrentEliasFanoBuilder<QUANTUM_LOG2> {
    fn default() -> ConcurrentEliasFanoBuilder<QUANTUM_LOG2> {
        ConcurrentEliasFanoBuilder {
            high_bits: SparseIndex::new_concurrent(0, 0),
            low_bits: CompactArray::new(1),
            number_of_elements: 0,
            universe: 0,
        }
    }
}

impl<const QUANTUM_LOG2: usize> ConcurrentEliasFanoBuilder<QUANTUM_LOG2> {
    /// Initialize a new concurrent builder for ELias-Fano
    pub fn new(
        number_of_elements: usize,
        universe: usize,
    ) -> Result<ConcurrentEliasFanoBuilder<QUANTUM_LOG2>, String> {
        // If the user says that there will be no elements, the builder will
        // only use the high-bits to store the eventual values
        if number_of_elements == 0 {
            return Ok(ConcurrentEliasFanoBuilder::default());
        }

        use core::intrinsics::{ceilf64, floorf64, log2f64};
        // Compute the size of the low bits.
        let low_bit_count = if universe >= number_of_elements as usize {
            unsafe { floorf64(log2f64(universe as f64 / number_of_elements as f64)) as usize }
        } else {
            0
        };

        // saturate at the max we can handle
        if low_bit_count > WORD_BIT_SIZE {
            return Err(format!(concat!(
                    "The lowbits are too big, in EliasFano we only support 64 bits for the low parts.",
                    "The value were universe {} number_of_elements {}"
                ),
                universe, number_of_elements
            ));
        }

        // add 2 to do the ceil and have brenchless primitives.
        let low_bits = CompactArray::with_capacity(low_bit_count, number_of_elements as usize);

        // the number of bits will be at max the number of elements + max(high)
        // we need a ceil, but >> is floor so we add 1
        let high_size = unsafe {
            ceilf64(
                (number_of_elements + (universe >> low_bit_count)) as f64 / WORD_BIT_SIZE as f64,
            ) as usize
        };
        let high_bits = SparseIndex::new_concurrent(high_size, number_of_elements);

        Ok(ConcurrentEliasFanoBuilder {
            low_bits,
            high_bits,
            universe,
            number_of_elements: number_of_elements as usize,
        })
    }

    /// Write the given value in the elias-fano, this method is
    /// safe from concurrency and allows to build elias-fano in parallel
    /// if the indices of the values are known in advance.
    pub fn set(&self, index: usize, value: usize) {
        let high = value >> self.low_bits.word_size();
        let low = value & self.low_bits.word_mask();

        // write the low-bits
        self.low_bits.concurrent_write(index, low);

        // write the high-bits
        let idx = high + index;
        self.high_bits.set(idx);
    }

    ///  Consume the builder and returns the built EliasFano struct.
    /// This step is not really parallel and will have to build the
    /// high-bits indices needed for the constant time select.
    pub fn build(self) -> Result<EliasFano<QUANTUM_LOG2>, String> {
        let low_bit_count = self.low_bits.word_size();

        let mut result = EliasFano {
            low_bits: self.low_bits,
            high_bits: self.high_bits.build()?,
            universe: self.universe,
            number_of_elements: self.number_of_elements,

            // We assume that this is correct, we could check this but it would
            // mean that every thread should update it, thus possibily creating
            // a concurrency bottleneck.
            current_number_of_elements: self.number_of_elements,

            // These values are used to be able to push new values in the future.
            // We will initialize them to garbage, and then use the rank and select
            // methods to compute the right values
            last_high_value: 0,
            last_value: 0,
            last_index: self.number_of_elements,
        };

        if self.number_of_elements > 0 {
            let max_value = result
                .select(self.number_of_elements.saturating_sub(1))
                .unwrap();
            result.last_value = max_value;
            result.last_high_value = max_value >> low_bit_count;
        }

        Ok(result)
    }
}
