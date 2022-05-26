//! # Zeta code
//! This code is used in webgraph to store the deltas between neighbours, it's
//! basically the unary code of the bucket (golomb like but with power of twos),
//! followed by the minimal binary encoding of which value within the bucket we
//! have.
//!
//! # Example
//! ```rust
//! use elias_fano_rust::prelude::*;
//!
//! let mut ba = BitArrayM2LReader::new();
//!
//! // write values to the stream
//! for i in 0..100 {
//!     let idx = ba.tell_bits().unwrap();
//!
//!     // write the value
//!     ba.write_zeta::<8>(i).unwrap();
//!
//!     // ensure that size is consistent with the seek forwarding
//!     assert_eq!(ba.tell_bits().unwrap(), idx + ba.size_zeta::<8>(i) as usize);
//! }
//! // rewind the stream
//! ba.seek_bits(0).unwrap();
//!
//! // read back the values
//! for i in 0..100 {
//!     assert_eq!(i, ba.read_zeta::<8>().unwrap());
//! }
//!
//! let expected_size: usize = (0..100).map(|x| ba.size_zeta::<8>(x)).sum();
//! assert_eq!(expected_size, ba.tell_bits().unwrap())
//! ```
use super::*;
use super::tables::ZETA3_M2L_TABLE;
use crate::utils::{fast_log2_floor, fast_pow_2};
use crate::Result;

/// Trait for reading a zeta code (with K known at compile time)
pub trait CodeReadZeta: CodeReadUnary + CodeReadMinimalBinary + ReadBit {
    #[inline]
    /// Read a Zeta code from the stream
    fn read_zeta<const K: usize>(&mut self) -> Result<usize> {
        // check if the value is in one of the tables
        #[cfg(feature = "code_tables")]
        if K == 3 {
            let (res, len) = ZETA3_M2L_TABLE[self.peek_byte()? as usize];
            // if the value was in the table, return it and offset
            if len != 0 {
                self.skip_bits(len as usize)?;
                return Ok(res as usize)
            }
    
        }
        // fallback to the actual implementation
        let h = self.read_unary()?;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);
        let r = self.read_minimal_binary(u - l)?;
        Ok(l + r - 1)
    }
}

/// Trait for writing a zeta code (with K known at compile time)
pub trait CodeWriteZeta: CodeWriteUnary + CodeWriteMinimalBinary {
    #[inline]
    /// Write a Zeta code to the stream
    fn write_zeta<const K: usize>(&mut self, mut value: usize) -> Result<()> {
        value += 1;
        let h = fast_log2_floor(value) / K;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);

        debug_assert!(l <= value, "{} <= {}", l, value);
        debug_assert!(value < u, "{} < {}", value, u);

        self.write_unary(h)?;
        self.write_minimal_binary(value - l, u - l)
    }
}

/// Trait for the size of a zeta code with K known at compile time
pub trait CodeSizeZeta: CodeSizeUnary + CodeSizeMinimalBinary {
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_zeta<const K: usize>(&mut self, mut value: usize) -> usize {
        value += 1;
        let h = fast_log2_floor(value) / K;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);
        self.size_unary(h) + self.size_minimal_binary(value - l, u - l)
    }
}

/// blanket implementation
impl<T: CodeReadUnary + CodeReadMinimalBinary> CodeReadZeta for T {}
impl<T: CodeWriteUnary + CodeWriteMinimalBinary> CodeWriteZeta for T {}
impl<T: CodeSizeUnary + CodeSizeMinimalBinary> CodeSizeZeta for T {}
