//! Golomb implementation for runtime dispatching, this is only **slighly slower**
//! than the compile time version
use super::*;
use crate::utils::fast_log2_ceil;
use crate::Result;

/// Read a golomb code with `B` known at runtime
pub trait CodeReadGolombRuntime: CodeReadUnary + CodeReadFixedLength {
    #[inline]
    /// Read a golomb code from the stream
    #[allow(non_snake_case)]
    fn read_golomb_runtime(&mut self, B: usize) -> Result<usize> {
        let blocks_count = self.read_unary()?;
        Ok(blocks_count * B + self.read_fixed_length(fast_log2_ceil(B))?)
    }
}

/// Write a golomb code with `B` known at runtime
pub trait CodeWriteGolombRuntime: CodeWriteUnary + CodeWriteFixedLength {
    #[inline]
    /// Write a golomb code to the stream
    #[allow(non_snake_case)]
    fn write_golomb_runtime(&mut self, value: usize, B: usize) -> Result<()> {
        self.write_unary(value / B)?;
        self.write_fixed_length(fast_log2_ceil(B), value % B)?;
        Ok(())
    }
}

/// blanket implementation
impl<T: CodeReadUnary + CodeReadFixedLength> CodeReadGolombRuntime for T {}
impl<T: CodeWriteUnary + CodeWriteFixedLength> CodeWriteGolombRuntime for T {}
