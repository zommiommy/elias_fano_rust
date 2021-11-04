use crate::traits::CoreIoError;

/// Trait for handling unary codes
pub trait CodeUnary {
    /// Read unary code from the stream.
    /// 
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn read_unary(&mut self) -> Result<usize, CoreIoError>;

    /// Write unary code to the stream.
    /// 
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn write_unary(&mut self, value: usize) -> Result<(), CoreIoError>;

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_unary(&mut self, value: usize) -> usize {
        value + 1
    }
}