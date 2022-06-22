//! This assumes that the values are read from the MSB to the LSB
use super::fixed_length::*;
use crate::traits::ReadBit;
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::Result;

/// Write a minimal binary code that's read form the MSB to the LSB
pub trait CodeReadMinimalBinarym2l: CodeReadFixedLength + ReadBit {
    #[inline]
    /// Read a minimal binary value (BV) from the stream
    fn read_minimal_binary_m2l(&mut self, max: usize) -> Result<usize> {
        let u = fast_log2_ceil(max);
        let l = fast_log2_floor(max);
        let mut n = self.read_fixed_length(l)?;
        let scarto = fast_pow_2(u) - max;

        Ok(if n < scarto {
            n
        } else {
            n <<= 1;
            n += self.read_bit()? as usize;
            n - 1
        })
    }
}

/// Read a minimal binary code that's read form the MSB to the LSB
pub trait CodeWriteMinimalBinarym2l: CodeWriteFixedLength {
    #[inline]
    /// Write a minimal binary (BV) value from the stream
    fn write_minimal_binary_m2l(&mut self, value: usize, max: usize) -> Result<()> {
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
impl<T: ReadBit + CodeReadFixedLength> CodeReadMinimalBinarym2l for T {}
impl<T: CodeWriteFixedLength> CodeWriteMinimalBinarym2l for T {}
