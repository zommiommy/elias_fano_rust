use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use super::fixed_length::CodeFixedLength;
use crate::traits::*;

/// Huffman Optimal code for uniform distribution
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
///     ba.write_minimal_binary(i, max).unwrap();
/// 
/// 
///     // ensure that size is consistent with the seek forwarding
///     assert_eq!(
///         ba.tell_bits().unwrap(), 
///         idx + ba.size_minimal_binary(i, max)
///     );
/// }
/// 
/// // rewind the stream
/// ba.seek_bits(0).unwrap();
/// 
/// // read back the values
/// for i in 0..max {
///     assert_eq!(i, ba.read_minimal_binary(max).unwrap());
/// }
///
pub trait CodeMinimalBinary: CodeFixedLength + ReadBit {

    #[inline]
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize, CoreIoError> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let n = self.read_fixed_length(l)?;
        let scarto = fast_pow_2(u) - max; 
        
        if n  < scarto {
            return Ok(n);
        } 
        // rewind to read the code again
        self.rewind_bits(l as _)?;
        // decode the value
        let r = self.read_fixed_length(u)?;

        if r < fast_pow_2(l) {
            Ok(r)
        } else {
            Ok(r - scarto)
        }
    }

    #[inline]
    fn write_minimal_binary(&mut self, value: usize, max: usize) -> Result<(), CoreIoError> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;

        if value < scarto {
            self.write_fixed_length(l, value)
        } else if value < fast_pow_2(l) {
            self.write_fixed_length(u, value)
        } else {
            self.write_fixed_length(u, value + scarto)
        }
    }
    
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_minimal_binary(&mut self, value: usize, max: usize) -> usize {
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
impl<T: ReadBit + CodeFixedLength> CodeMinimalBinary for T {}