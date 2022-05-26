//! Dinamic version of the zeta code, this is **slower** than the constant version
//! but can be dispatched at runtime.
use super::*;
use super::tables::ZETA3_M2L_TABLE;
use crate::utils::{fast_log2_floor, fast_pow_2};
use crate::Result;

/// Read a zeta code with K known at runtime
pub trait CodeReadZetaRuntime: CodeReadUnary + CodeReadMinimalBinary {
    #[inline]
    /// Read a Zeta code from the stream
    #[allow(non_snake_case)]
    fn read_zeta_runtime(&mut self, K: usize) -> Result<usize> {
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
        let h = self.read_unary()?;
        let u = fast_pow_2((h + 1) * K);
        let l = fast_pow_2(h * K);
        let r = self.read_minimal_binary(u - l)?;
        Ok(fast_pow_2(h * K) + r - 1)
    }
}

/// Write a zeta code with K known at runtime
pub trait CodeWriteZetaRuntime: CodeWriteUnary + CodeWriteMinimalBinary {
    #[inline]
    /// Write a Zeta code to the stream
    #[allow(non_snake_case)]
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
