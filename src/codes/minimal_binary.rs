use super::fixed_length::*;
use crate::traits::*;
use crate::utils::{fast_log2_ceil, fast_log2_floor, fast_pow_2};
use crate::CodeReadMinimalBinaryBig;
use crate::Result;

/// Huffman Optimal code for uniform distribution,
/// as described by Boldi and Vigna
pub trait CodeReadMinimalBinary {
    /// Read a minimal binary value (BV) from the stream
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize>;
}

pub trait CodeWriteMinimalBinary {
    /// Write a minimal binary value from the stream
    fn write_minimal_binary(&mut self, value: usize, max: usize) -> Result<()>;
}

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

impl<T> CodeReadMinimalBinary for T
where
    T: CodeReadFixedLength + ReadBit + IsBigEndian<true>,
{
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.read_minimal_binary_big(max)
    }
}

impl<T> CodeWriteMinimalBinary for T
where
    T: CodeWriteFixedLength + WriteBit + IsBigEndian<true>,
{
    fn write_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.write_minimal_binary_big(max)
    }
}

impl<T> CodeReadMinimalBinary for T
where
    T: CodeReadFixedLength + ReadBit + IsBigEndian<false>,
{
    fn read_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.read_minimal_binary_little(max)
    }
}

impl<T> CodeWriteMinimalBinary for T
where
    T: CodeWriteFixedLength + WriteBit + IsBigEndian<false>,
{
    fn write_minimal_binary(&mut self, max: usize) -> Result<usize> {
        self.write_minimal_binary_little(max)
    }
}
