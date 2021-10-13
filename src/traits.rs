/// Get info about the data
pub trait MemoryFootprint {
    /// Returns the number of bytes it uses
    fn total_size(&self) -> usize;
}