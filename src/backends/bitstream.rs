
use crate::traits::*;
use crate::constants::*;
use crate::codes::{CodeUnary, CodeFixedLength};
use core::mem::size_of;
use crate::Result;

/// A general bitstream wrapper over some word reader and writers.
/// The goal of this wrapper is to be able to provide a generic Monad:
/// `T -> R | T: Read + Write => R: ReadBit + WriteBit + CodeUnary + CodeFixedLength`
pub struct BitStream<BACKEND: Write + Read + MemoryFootprint> {
    /// The actual word reader / writer
    pub backend: BACKEND,
    /// A small buffer to be able to handle unaligned reads or
    /// writes
    pub buffer: [usize; 2],
    pub bit_index: usize,
}

impl<BACKEND: Write + Read + MemoryFootprint> MemoryFootprint for BitStream<BACKEND> {
    fn total_size(&self) -> usize {
        self.backend.total_size()
        + 4 * size_of::<usize>()
    }
}

impl<BACKEND: Write + Read + MemoryFootprint> BitStream<BACKEND> {
    pub fn new(backend: BACKEND) -> BitStream<BACKEND> {
        BitStream{
            backend,
            buffer: [0, 0],
            bit_index: 0,
        }
    }

    #[inline]
    /// Destroy the Bitstream wrapper and return the inner backend
    pub fn into_inner(self) -> BACKEND {
        self.backend
    }
}

impl<BACKEND: Write + Read + MemoryFootprint> ReadBit for BitStream<BACKEND>  {
    #[inline]
    /// Read a single bit
    fn read_bit(&mut self) -> Result<bool> {
        
    }

    #[inline]
    /// Seek to the given bit_index
    fn seek_bits(&mut self, bit_index: usize) -> Result<()> {
        self.bit_index = bit_index;
    }

    #[inline]
    /// Return the current position (bit index) in the bit array
    fn tell_bits(&self) -> Result<usize> {

    }

    #[inline]
    /// Overriding optimized version
    fn skip_bits(&mut self, bit_offset: usize) -> Result<()> {

    }    
}

impl<BACKEND: Write + Read + MemoryFootprint> WriteBit for BitStream<BACKEND>  {
    #[inline]
    /// Read a single bit
    fn write_bit(&mut self, value: bool) -> Result<()>{

    }

    #[inline]
    /// For a bitarray there is not need to flush anything
    fn flush_bits(&mut self) {
        self.backend.flush();
    }
}

/// Optimal for gemetric distribution of ratio 1/2
impl<BACKEND: Write + Read + MemoryFootprint> CodeUnary for BitStream<BACKEND> {
    #[inline]
    fn read_unary(&mut self) -> Result<usize> {

    }

    #[inline]
    fn write_unary(&mut self, value: usize) -> Result<()> {
        
    }

}

/// Optimal for uniform distribution
impl<BACKEND: Write + Read + MemoryFootprint> CodeFixedLength for BitStream<BACKEND> {
    #[inline]
    /// Read `number_of_bits` from the stream.
    /// THIS SHOULD NOT BE CALLED WITH `number_of_bits` equal to 0.
    fn read_fixed_length(&mut self, number_of_bits: usize) -> Result<usize> {

    }

    #[inline]
    /// Write `value` using `number_of_bits` in the stream.
    fn write_fixed_length(&mut self, number_of_bits: usize, value: usize) -> Result<()> {

    }
}