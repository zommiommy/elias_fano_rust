//! # Golomb Code
//! Optimal for gemetric distribution of ratio:
//! $$\frac{1}{\sqrt^b{2}$$
//!
//! # Example
//! ```rust
//! use elias_fano_rust::prelude::*;
//!
//! let mut ba = BitArrayLittle::new();
//!
//! // write values to the stream
//! for i in 0..100 {
//!     let idx = ba.tell_bits().unwrap();
//!
//!     // write the value
//!     ba.write_golomb::<8>(i).unwrap();
//!
//!     // ensure that size is consistent with the seek forwarding
//!     assert_eq!(ba.tell_bits().unwrap(), idx + ba.size_golomb::<8>(i));
//! }
//! // rewind the stream
//! ba.seek_bits(0).unwrap();
//!
//! // read back the values
//! for i in 0..100 {
//!     assert_eq!(i, ba.read_golomb::<8>().unwrap());
//! }
//!
//! let expected_size: usize = (0..100).map(|x| ba.size_golomb::<8>(x)).sum();
//! assert_eq!(expected_size, ba.tell_bits().unwrap())
//! ```
use super::{fixed_length::*, unary::*};
use crate::utils::fast_log2_ceil;
use crate::Result;

#[inline]
/// Given the ratio `p` of a geometric distribution
/// compute the optimal golomb block size
///
/// # Safety
/// This could fail if LLVM does not support floating values for the current
/// arch
pub fn compute_optimal_golomb_block_size(p: f64) -> usize {
    use core::intrinsics::{ceilf64, log2f64};
    unsafe { ceilf64(-log2f64(2.0 - p) / log2f64(1.0 - p)) as usize }
}

/// Read a golomb code
pub trait CodeReadGolomb: CodeReadUnary + CodeReadFixedLength {
    #[inline]
    /// Read a golomb code from the stream
    fn read_golomb<const B: usize>(&mut self) -> Result<usize> {
        let blocks_count = self.read_unary()?;
        Ok(blocks_count * B + self.read_fixed_length(fast_log2_ceil(B))?)
    }
}

/// Write a golomb code
pub trait CodeWriteGolomb: CodeWriteUnary + CodeWriteFixedLength {
    #[inline]
    /// Write a golomb code to the stream
    fn write_golomb<const B: usize>(&mut self, value: usize) -> Result<()> {
        self.write_unary(value / B)?;
        self.write_fixed_length(fast_log2_ceil(B), value % B)?;
        Ok(())
    }
}

/// Get the size in bits of the golomb encoding of a given value
pub trait CodeSizeGolomb: CodeSizeUnary + CodeSizeFixedLength {
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_golomb<const B: usize>(&mut self, value: usize) -> usize {
        self.size_unary(value / B) + self.size_fixed_length(fast_log2_ceil(B))
    }
}

/// blanket implementation
impl<T: CodeReadUnary + CodeReadFixedLength> CodeReadGolomb for T {}
impl<T: CodeWriteUnary + CodeWriteFixedLength> CodeWriteGolomb for T {}
impl<T: CodeSizeUnary + CodeSizeFixedLength> CodeSizeGolomb for T {}
