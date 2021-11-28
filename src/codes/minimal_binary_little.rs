//! This assumes that the values are read from the LSB to the MSB (as in little endian)
use super::fixed_length::*;
use crate::traits::*;
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::Result;

/// Read a minimal binary little endian code
pub trait CodeReadMinimalBinaryLittle: CodeReadFixedLength + ReadBit {
    #[inline]
    /// Read a minimal binary value from the stream
    fn read_minimal_binary_little(&mut self, max: usize) -> Result<usize> {
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

/// Write a minimal binary little endian code
pub trait CodeWriteMinimalBinaryLittle: CodeWriteFixedLength {
    #[inline]
    /// Write a minimal binary value from the stream
    fn write_minimal_binary_little(&mut self, value: usize, max: usize) -> Result<()> {
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
impl<T: ReadBit + CodeReadFixedLength> CodeReadMinimalBinaryLittle for T {}
impl<T: CodeWriteFixedLength> CodeWriteMinimalBinaryLittle for T {}
