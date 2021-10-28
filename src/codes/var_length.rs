use crate::utils::fast_log2_ceil;
use crate::traits::CoreIoError;
use super::{
    unary::CodeUnary, 
    fixed_length::CodeFixedLength,
};

/// Var length encoding, this is the non-interleaved version of
/// LSB128 encoding.
/// 
/// TODO!: Implement LSB128, it might be faster thanks to 
/// PMOVMSKB SSE instruciton
/// 
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
/// 
/// let mut bs = BitArray::new();
/// 
/// // write values to the stream
/// for i in 0..100 {
///     let idx = bs.tell_bits().unwrap();
/// 
///     // write the value
///     bs.write_var_length::<3>(i).unwrap();
/// 
///     // ensure that size is consistent with the seek forwarding
///     assert_eq!(bs.tell_bits().unwrap(), idx + bs.size_var_length::<3>(i));
/// }
/// // rewind the stream
/// bs.seek_bits(0).unwrap();
/// 
/// // read back the values
/// for i in 0..100 {
///     assert_eq!(i, bs.read_var_length::<3>().unwrap());
/// }
/// ```
pub trait CodeVarLength: CodeUnary + CodeFixedLength {
    #[inline]
    fn read_var_length<const K: usize>(&mut self) -> Result<usize, CoreIoError> {
        let len = self.read_unary()?;

        // read `len` blocks of `K` bits
        // on x86_64 this constant multiplication will be converted
        // to a LEA instruction which is MUCH faster than a MUL
        self.read_fixed_length(K * len)
    }

    #[inline]
    fn write_var_length<const K: usize>(&mut self, value: usize) -> Result<(), CoreIoError> {
        use core::intrinsics::ceilf64;

        // compute how many blocks of `K` bits we are going to use
        let number_of_blocks_to_write = unsafe{ceilf64(fast_log2_ceil(value + 1) as f64 / K as f64)} as usize;
        self.write_unary(number_of_blocks_to_write)?;

        // write `prefix` blocks of `K` bits
        self.write_fixed_length(K * number_of_blocks_to_write, value)
    }

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_var_length<const K: usize>(&mut self, value: usize) -> usize {
        use core::intrinsics::ceilf64;

        let number_of_blocks_to_write = unsafe{ceilf64(fast_log2_ceil(value + 1) as f64 / K as f64)} as usize;
        self.size_unary(number_of_blocks_to_write) 
            + self.size_fixed_length(K * number_of_blocks_to_write)
    }
}

/// blanket implementation
impl<T: CodeUnary + CodeFixedLength> CodeVarLength for T {}