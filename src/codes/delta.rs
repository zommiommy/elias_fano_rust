//! # Elias’ δ
//! universal coding of x ∈ N+ is obtained by representing x in binary
//! preceded by a representation of its length in γ.
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
//!     ba.write_delta(i).unwrap();
//!
//!     // ensure that size is consistent with the seek forwarding
//!     assert_eq!(ba.tell_bits().unwrap(), idx + ba.size_delta(i));
//! }
//! // rewind the stream
//! ba.seek_bits(0).unwrap();
//!
//! // read back the values
//! for i in 0..100 {
//!     assert_eq!(i, ba.read_delta().unwrap());
//! }
//!
//! let expected_size: usize = (0..100).map(|x| ba.size_delta(x)).sum();
//! assert_eq!(expected_size, ba.tell_bits().unwrap());
use super::{gamma::*, unary::*};
use crate::utils::fast_log2_ceil;
use crate::Result;

/// Read a Delta Code
pub trait CodeReadDelta: CodeReadUnary + CodeReadGamma {
    #[inline]
    /// Read a delta code from the stream
    fn read_delta(&mut self) -> Result<usize> {
        let len = self.read_gamma()?;
        self.read_fixed_length(len)
    }
}

/// Write a Delta Code
pub trait CodeWriteDelta: CodeWriteUnary + CodeWriteGamma {
    #[inline]
    /// Write a delta code to the stream
    fn write_delta(&mut self, value: usize) -> Result<()> {
        // TODO!: figure out if the +1 is actually needed
        let number_of_blocks_to_write = fast_log2_ceil(value + 1);
        self.write_gamma(number_of_blocks_to_write)?;
        self.write_fixed_length(number_of_blocks_to_write, value)?;
        Ok(())
    }
}

/// Size of a given value a Delta Code
pub trait CodeSizeDelta: CodeSizeUnary + CodeSizeGamma {
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_delta(&mut self, value: usize) -> usize {
        let number_of_blocks_to_write = fast_log2_ceil(value + 1);
        self.size_gamma(number_of_blocks_to_write)
            + self.size_fixed_length(number_of_blocks_to_write)
    }
}

/// blanket implementation
impl<T: CodeReadUnary + CodeReadGamma> CodeReadDelta for T {}
impl<T: CodeWriteUnary + CodeWriteGamma> CodeWriteDelta for T {}
impl<T: CodeSizeUnary + CodeSizeGamma> CodeSizeDelta for T {}
