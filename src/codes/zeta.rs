use crate::utils::{
    fast_log2_floor,
    fast_pow_2,
};
use crate::traits::CoreIoError;
use super::{
    unary::CodeUnary, 
    minimal_binary::CodeMinimalBinary,
};

///
/// 
/// # Example
/// ```rust
/// use elias_fano_rust::prelude::*;
/// 
/// let mut ba = BitArray::new();
/// 
/// // write values to the stream
/// for i in 0..100 {
///     let idx = ba.tell_bits().unwrap();
/// 
///     // write the value
///     ba.write_zeta::<8>(i).unwrap();
/// 
///     // ensure that size is consistent with the seek forwarding
///     assert_eq!(ba.tell_bits().unwrap(), idx + ba.size_zeta::<8>(i) as usize);
/// }
/// // rewind the stream
/// ba.seek_bits(0).unwrap();
/// 
/// // read back the values
/// for i in 0..100 {
///     assert_eq!(i, ba.read_zeta::<8>().unwrap());
/// }
/// ```
pub trait CodeZeta: CodeUnary + CodeMinimalBinary {

    #[inline]
    fn read_zeta<const K: usize>(&mut self) -> Result<usize, CoreIoError> {
        let h = self.read_unary()?;
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);
        let r = self.read_minimal_binary(u - l)?;
        Ok(fast_pow_2(h * K) + r - 1)
    }

    #[inline]
    fn write_zeta<const K: usize>(&mut self, mut value: usize) -> Result<(), CoreIoError> {
        value += 1;
        let h = fast_log2_floor(value) / K; // wtf
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);

        debug_assert!(l <= value, "{} <= {}", l, value);
        debug_assert!(value < u, "{} < {}", value, u);

        self.write_unary(h)?;
        self.write_minimal_binary( value - fast_pow_2(h * K),  u - l)
    }

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_zeta<const K: usize>(&mut self, mut value: usize) -> usize {
        value += 1;
        let h = fast_log2_floor(value) / K; // wtf
        let u = fast_pow_2((h + 1) * K);
        let l =  fast_pow_2(h * K);
        self.size_unary(h) 
            + self.size_minimal_binary(value - fast_pow_2(h * K),  u - l)
    }
}

/// blanket implementation
impl<T: CodeUnary + CodeMinimalBinary> CodeZeta for T {}