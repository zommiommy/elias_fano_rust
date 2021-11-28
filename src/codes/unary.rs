use crate::traits::*;
use crate::Result;

/// Trait for reading unary codes
pub trait CodeReadUnary: ReadBit {
    /// Read unary code from the stream.
    ///
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn read_unary(&mut self) -> Result<usize> {
        let mut value = 0;
        loop {
            if self.read_bit()? {
                return Ok(value);
            }
            value += 1;
        }
    }
}

/// Trait for writing unary codes
pub trait CodeWriteUnary: WriteBit {
    /// Write unary code to the stream.
    ///
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn write_unary(&mut self, value: usize) -> Result<()> {
        for _ in 0..value {
            self.write_bit(false)?;
        }
        self.write_bit(true)?;
        Ok(())
    }
}

/// Trait for structures to compute the size of the unary code.
pub trait CodeSizeUnary {
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_unary(&mut self, value: usize) -> usize {
        value + 1
    }
}
