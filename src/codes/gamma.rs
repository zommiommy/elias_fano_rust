use crate::utils::fast_log2_floor;
use crate::traits::CoreIoError;
use super::{
    unary::CodeUnary, 
    fixed_length::CodeFixedLength,
};

/// Optimal for Zipf of exponent 2
/// Elias’ γ universal coding of x ∈ N+ is obtained by representing x in binary
// preceded by a unary representation of its length (minus one).
// More precisely, to represent x we write in unary floor(log(x)) and then in
// binary x - 2^ceil(log(x)) (on floor(log(x)) bits)
/// 
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
/// 
/// let mut ba = BitArrayLittle::new();
/// 
/// // write values to the stream
/// for i in 0..100 {
///     let idx = ba.tell_bits().unwrap();
/// 
///     // write the value
///     ba.write_gamma(i).unwrap();
/// 
///     // ensure that size is consistent with the seek forwarding
///     assert_eq!(ba.tell_bits().unwrap(), idx + ba.size_gamma(i));
/// }
/// 
/// // rewind the stream
/// ba.seek_bits(0).unwrap();
/// 
/// // read back the values
/// for i in 0..100 {
///     assert_eq!(i, ba.read_gamma().unwrap());
/// }
/// 
/// let expected_size: usize = (0..100).map(|x| ba.size_gamma(x)).sum();
/// assert_eq!(expected_size, ba.tell_bits().unwrap())
/// ```
pub trait CodeGamma: CodeUnary + CodeFixedLength {

    #[inline]
    /// Read a gamma code from the stream
    fn read_gamma(&mut self) -> Result<usize, CoreIoError> {
        let len = self.read_unary()?;
        Ok(self.read_fixed_length(len)? + (1 << len) - 1)
    }

    #[inline]
    /// Write a gamma code to the stream
    fn write_gamma(&mut self, mut value: usize) -> Result<(), CoreIoError> {
        value += 1;
        let number_of_blocks_to_write = fast_log2_floor(value);
        // remove the most significant 1
        let short_value = value - (1 << number_of_blocks_to_write);
        // TODO this can be optimized 
        // Write the code
        self.write_unary(number_of_blocks_to_write)?;
        self.write_fixed_length(number_of_blocks_to_write, short_value)?;
        Ok(())
    }

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_gamma(&mut self, mut value: usize) -> usize {
        value += 1;
        let number_of_blocks_to_write = fast_log2_floor(value);
        self.size_unary(number_of_blocks_to_write) 
            + self.size_fixed_length(number_of_blocks_to_write)
    }
}

/// blanket implementation
impl<T: CodeUnary + CodeFixedLength> CodeGamma for T {}