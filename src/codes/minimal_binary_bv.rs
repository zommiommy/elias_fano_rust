use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use super::fixed_length::CodeFixedLength;
use crate::traits::{ReadBit, CoreIoError};

#[inline(always)]
/// Reverse the bits in a word long `number_of_bits`.
/// This assumes that `number_of_bits <=  8 * core::mem::size_of::<usize>()`
/// 
/// ```ignore
/// // test on that this is an involution
///  for i in 0..1_000 {
///     assert_eq!(i, reverse_bits(reverse_bits(i, 10), 10));
/// }
/// 
/// assert_eq!(0b1010, reverse_bits(reverse_bits(0b0101, 4), 4))
/// ```
fn reverse_bits(value: usize, number_of_bits: usize) -> usize {
    const WORD_BIT_SIZE: usize = 8 * core::mem::size_of::<usize>();
    debug_assert!(WORD_BIT_SIZE >= number_of_bits);
    value.reverse_bits() >> (WORD_BIT_SIZE - number_of_bits)
}

/// Huffman Optimal code for uniform distribution,
/// as described by Boldi and Vigna
/// 
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
/// 
/// let mut ba = BitArray::new();
/// 
/// let max = 1_000;
/// // write values to the stream
/// for i in 0..max {
///     let idx = ba.tell_bits().unwrap();
/// 
///     // write the value
///     ba.write_minimal_binary_bv(i, max).unwrap();
/// 
/// 
///     // ensure that size is consistent with the seek forwarding
///     assert_eq!(
///         ba.tell_bits().unwrap(), 
///         idx + ba.size_minimal_binary_bv(i, max)
///     );
/// }
/// 
/// // rewind the stream
/// ba.seek_bits(0).unwrap();
/// 
/// // read back the values
/// for i in 0..max {
///     assert_eq!(i, ba.read_minimal_binary_bv(max).unwrap());
/// }
/// 
/// let expected_size: usize = (0..max).map(|x| ba.size_minimal_binary_bv(x, max)).sum();
/// assert_eq!(expected_size, ba.tell_bits().unwrap())
/// ```
///
pub trait CodeMinimalBinaryBV: CodeFixedLength + ReadBit {
    #[inline]
    fn read_minimal_binary_bv(&mut self, max: usize) -> Result<usize, CoreIoError> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let n = reverse_bits(self.read_fixed_length(l)?, l);
        let scarto = fast_pow_2(u) - max; 
        
        if n  < scarto {
            return Ok(n);
        } 
        // rewind to read the code again
        self.rewind_bits(l as _)?;
        // decode the value
        let r = reverse_bits(self.read_fixed_length(u)?, u);
        Ok(r - scarto)
    }

    #[inline]
    fn write_minimal_binary_bv(&mut self, value: usize, max: usize) -> Result<(), CoreIoError> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;

        if value < scarto {
            self.write_fixed_length(l, reverse_bits(value, l))
        } else {
            self.write_fixed_length(u, reverse_bits(value + scarto, u))
        }
    }

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_minimal_binary_bv(&mut self, value: usize, max: usize) -> usize {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;
        
        if value < scarto {
            l
        } else {
            u
        }
    }
}

/// blanket implementation
impl<T: ReadBit + CodeFixedLength> CodeMinimalBinaryBV for T {}