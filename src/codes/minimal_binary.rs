//! # Minimal Binary Code
//! 
//! Huffman Optimal code for uniform distribution of values between 0 and max.
//! 
//! We offer two implementation of this code, one for big endian code that assumes
//! that values are read form the MSB to the LSB, and a little endian code that
//! assumes that values are read from the LSB to the MSB. 
//! The common code is not immediate if reversed so we have two funtionally identical
//! codes, one for each case. A backend should just implement this trait using
//! one of the two available implementations, depending on it's endianess.
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::Result;

/// Trait for data strutures that can read minimal binary codes
pub trait CodeReadMinimalBinary {
    /// Read a minimal binary value (BV) from the stream
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize>;
}

/// Trait for data strutures that can Write minimal binary codes
pub trait CodeWriteMinimalBinary {
    /// Write a minimal binary value from the stream
    fn write_minimal_binary(&mut self, value: usize, max: usize) -> Result<()>;
}


/// Trait for data strutures that can compute the size in bits of minimal binary codes
/// (this should be a constant, but different datastructures might use more memory
/// for speed sake)
pub trait CodeSizeMinimalBinary {
    #[inline]
    /// Return how many bits the minimal binary code for the given value is long
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