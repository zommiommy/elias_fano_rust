use crate::*;
use crate::codes::*;

/// Small implementation of the `std::io::Write` trait that
/// for `#![no_std]` environments
pub trait Write {
    /// write a word of memory at the current offset, and seek forward by
    /// `core::mem::size_of::<usize>()` bytes.
    fn write(&mut self, word: usize) -> Result<()>;

    /// Flush the writes to the stream
    fn flush(&mut self);
}


/// A trait for a data-structure that can instantiate multiple writers 
/// (but only one at time can work)
pub trait BitWriter {
    /// The writer returend
    type WriterType: WriteBit + CodesWrite;
    /// Get a newly instantiated writer, there can only be one active at time.
    /// The writer will be initialized to the end of the stream (append mode)
    fn get_writer(&mut self) -> Self::WriterType;
}

/// A trait for a datastructure that can instantiate multiple readers
pub trait BitReader {
    /// The reader type
    type ReaderType: ReadBit + CodesRead;
    /// Get a new reader from the start of the stream
    fn get_reader(&self) -> Self::ReaderType;
}       

/// Small implementation of the `std::io::Read` trait that
/// for `#![no_std]` environments
pub trait Read {
    /// write a word of memory at the current offset, and seek forward by
    /// `core::mem::size_of::<usize>()` bytes.
    ///
    /// This method is mutable because it will seek forward and this is fundamentally
    /// a mutable operation which might lead to data races or general concurrency
    /// problems.
    fn read(&mut self) -> Result<usize>;

    /// Seek to the given offset, for performance reason we assume
    /// that seek is always word-aligned `seek % core::mem::size_of::<usize>() == 0`.
    /// If this condition is not respected the resulting behaviour is not defined
    fn seek(&mut self, word_offset: usize) -> Result<()>;

    /// Returns the current offset in the stream
    fn tell(&self) -> Result<usize>;

    /// Seek forward by `offset` words
    fn skip(&mut self, offset: usize) -> Result<()> {
        self.seek(self.tell()? + offset)
    }

    /// Seek backward by `offset` words, saturating at 0
    fn rewind(&mut self, offset: usize) -> Result<()> {
        self.seek(self.tell()?.saturating_sub(offset))
    }
}

/// Trait for structs that can write singular bits into a stream
pub trait WriteBit {
    /// write a word of memory at the current offset, and seek forward by
    /// 1 bit.
    fn write_bit(&mut self, bit: bool) -> Result<()>;

    /// Flush the writes to the stream
    fn flush_bits(&mut self);
}

/// Trait for structs that can read singular bits from a stream
pub trait ReadBit {
    /// write a word of memory at the current offset, and seek forward by
    /// 1 bit.
    ///
    /// This method is mutable because it will seek forward and this is fundamentally
    /// a mutable operation which might lead to data races or general concurrency
    /// problems.
    fn read_bit(&mut self) -> Result<bool>;

    /// Seek to the given bit offset
    fn seek_bits(&mut self, bit_offset: usize) -> Result<()>;

    /// Returns the current bit offset in the stream
    fn tell_bits(&self) -> Result<usize>;

    /// Seek forward by `bit_offset` bits
    fn skip_bits(&mut self, bit_offset: usize) -> Result<()> {
        self.seek_bits(self.tell_bits()? + bit_offset)
    }

    /// Seek backward by `bit_offset` bits, saturating at 0
    fn rewind_bits(&mut self, bit_offset: usize) -> Result<()> {
        self.seek_bits(self.tell_bits()?.saturating_sub(bit_offset))
    }
}
