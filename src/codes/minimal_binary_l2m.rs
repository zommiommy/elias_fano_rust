//! This assumes that the values are read from the LSB to the MSB (as in l2m endian)
use super::fixed_length::*;
use crate::traits::*;
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::Result;

/// Read a minimal binary code that's read form the LSB to the MSB
pub trait CodeReadMinimalBinaryl2m: CodeReadFixedLength + ReadBit {
    #[inline]
    /// Read a minimal binary value from the stream
    fn read_minimal_binary_l2m(&mut self, max: usize) -> Result<usize> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let n = self.read_fixed_length(l)?;
        let scarto = fast_pow_2(u) - max;

        if n < scarto {
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
}

/// Write a minimal binary code that's read form the LSB to the MSB
pub trait CodeWriteMinimalBinaryl2m: CodeWriteFixedLength {
    #[inline]
    /// Write a minimal binary value from the stream
    fn write_minimal_binary_l2m(&mut self, value: usize, max: usize) -> Result<()> {
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
}

/// blanket implementation
impl<T: ReadBit + CodeReadFixedLength> CodeReadMinimalBinaryl2m for T {}
impl<T: CodeWriteFixedLength> CodeWriteMinimalBinaryl2m for T {}
