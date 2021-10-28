use crate::traits::CoreIoError;

pub trait CodeFixedLength {
    /// Read  fixed length code from the stream.
    /// 
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize, CoreIoError> ;

    /// Write fixed length code to the stream.
    /// 
    /// This has a default implementation but
    /// it's **heavely suggested** to override it with a version optimized for
    /// your datastructure.
    fn write_fixed_length(&mut self, number_of_bits: usize, value: usize) -> Result<(), CoreIoError>;

    #[inline]
    /// Return how many bits the code for the given value is long
    fn size_fixed_length(&mut self, number_of_bits: usize) -> usize {
        number_of_bits
    }
}