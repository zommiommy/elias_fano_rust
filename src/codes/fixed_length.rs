use crate::traits::*;
use crate::Result;

/// General trait for objects that can write and read values with a fixed number
/// of bytes from a stream. This has no default implementation because it's a
/// fundamental primitive so the performance of the other codes heavely depends
/// on the optimizzation of these routins.
pub trait CodeReadFixedLength {
    /// Read  fixed length code from the stream.
    ///
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize>;
}

pub trait CodeWriteFixedLength {
    /// Write fixed length code to the stream.
    ///
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn write_fixed_length(&mut self, number_of_bits: usize, value: usize) -> Result<()>;
}

pub trait CodeSizeFixedLength {
    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_fixed_length(&mut self, number_of_bits: usize) -> usize {
        number_of_bits
    }
}
