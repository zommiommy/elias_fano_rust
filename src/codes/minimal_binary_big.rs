//! This assumes that the values are read from the MSB to the LSB (as in big endian)
use super::fixed_length::*;
use crate::traits::ReadBit;
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::Result;

/// Write a minimal binary big endian code
pub trait CodeReadMinimalBinaryBig: CodeReadFixedLength + ReadBit {
    #[inline]
    /// Read a minimal binary value (BV) from the stream
    fn read_minimal_binary_big(&mut self, max: usize) -> Result<usize> {
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
        Ok(r - scarto)
    }
}

/// Read a minimal binary big endian code
pub trait CodeWriteMinimalBinaryBig: CodeWriteFixedLength {
    #[inline]
    /// Write a minimal binary (BV) value from the stream
    fn write_minimal_binary_big(&mut self, value: usize, max: usize) -> Result<()> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let scarto = fast_pow_2(u) - max;

        if value < scarto {
            self.write_fixed_length(l, value)
        } else {
            self.write_fixed_length(u, value + scarto)
        }
    }
}

/// blanket implementation
impl<T: ReadBit + CodeReadFixedLength> CodeReadMinimalBinaryBig for T {}
impl<T: CodeWriteFixedLength> CodeWriteMinimalBinaryBig for T {}
