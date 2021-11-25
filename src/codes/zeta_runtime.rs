use super::*;
use crate::utils::{fast_log2_floor, fast_pow_2};
use crate::Result;

pub trait CodeReadZetaRuntime: CodeReadUnary + CodeReadMinimalBinary {
    #[inline]
    /// Read a Zeta code from the stream
    fn read_zeta_runtime(&mut self, K: usize) -> Result<usize> {
        let h = self.read_unary()?;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);
        let r = self.read_minimal_binary(u - l)?;
        Ok(fast_pow_2(h * K) + r - 1)
    }
}

pub trait CodeWriteZetaRuntime: CodeWriteUnary + CodeWriteMinimalBinary {
    #[inline]
    /// Write a Zeta code to the stream
    fn write_zeta_runtime(&mut self, mut value: usize, K: usize) -> Result<()> {
        value += 1;
        let h = fast_log2_floor(value) / K;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);

        debug_assert!(l <= value, "{} <= {}", l, value);
        debug_assert!(value < u, "{} < {}", value, u);

        self.write_unary(h)?;
        self.write_minimal_binary(value - fast_pow_2(h * K), u - l)
    }
}

/// blanket implementation
impl<T: CodeReadUnary + CodeReadMinimalBinary> CodeReadZetaRuntime for T {}
impl<T: CodeWriteUnary + CodeWriteMinimalBinary> CodeWriteZetaRuntime for T {}
