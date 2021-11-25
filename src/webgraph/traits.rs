use crate::codes::*;
use crate::traits::*;

pub trait WebGraphReaderBackend:
    ReadBit + CodeReadUnary + CodeReadFixedLength + MemoryFootprint
{
}
/// Blanket implementation
impl<T> WebGraphReaderBackend for T where
    T: ReadBit + CodeReadUnary + CodeReadFixedLength + MemoryFootprint
{
}

pub trait WebGraphWriterBackend:
    WriteBit + CodeWriteUnary + CodeWriteFixedLength + MemoryFootprint
{
}
/// Blanket implementation
impl<T> WebGraphWriterBackend for T where
    T: WriteBit + CodeWriteUnary + CodeWriteFixedLength + MemoryFootprint
{
}

pub trait WebGraphReaderCodesBackend: ReadBit {
    fn read_outdegrees(&mut self) -> Result<usize>;
    fn read_blocks(&mut self) -> Result<usize>;
    fn read_residuals(&mut self) -> Result<usize>;
    fn read_references(&mut self) -> Result<usize>;
    fn read_block_count(&mut self) -> Result<usize>;
    fn read_offsets(&mut self) -> Result<usize>;
}

pub trait WebGraphWriterCodesBackend: WriteBit {
    fn write_outdegrees(&mut self, value: usize) -> Result<()>;
    fn write_blocks(&mut self, value: usize) -> Result<()>;
    fn write_residuals(&mut self, value: usize) -> Result<()>;
    fn write_references(&mut self, value: usize) -> Result<()>;
    fn write_block_count(&mut self, value: usize) -> Result<()>;
    fn write_offsets(&mut self, value: usize) -> Result<()>;
}
