//! Collection of In-memory structs, or wrappers on which we can write and read
//! codes.
//!
//! `BitArrayM2LReader` and `BitArrayBig` allow to convert any indexable structure
//! to one that can use all the codes.
//!
//! `BitArrayM2LReader` read words from the LSB to the MSB, this is really hard to
//! make compatible across systems (the same data would be represented
//! differently on systems with different word widths).
//! Doing so require less logic and this is faster, its main goal is to get
//! better performance when inter-operability is not required.
//!
//! `BitArrayBig` read the bits from the MSB to the LSB, this is slower than
//! the other way around, but this code will create exactly the same results
//! on every machine.
//!
//! Both of these can both be supported by an in-memory vector or by an mmap-ed
//! (MapViewOfFile on windows) file in memory in order to support external memory.
//!
//! Finally, there is the BitStream wrapper that take anything that can read and
//! write words (a file, a socket, ...) and implements all the codes for it.
//! This allows for full generalizzation, and possibly distributed / over the
//! network structs, at the cost of more performance overhead.

pub use mmap::*;

// mod bitarray_little;
// pub use bitarray_little::*;

mod bitarray_m2l;
pub use bitarray_m2l::*;

// mod file_backend;
// pub use file_backend::*;

// mod bitstream;
// pub use bitstream::*;

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
    /// read a word of memory at the current offset, and seek forward by
    /// 1 bit.
    ///
    /// This method is mutable because it will seek forward and this is 
    /// fundamentally a mutable operation which might lead to data races or 
    /// general concurrency problems.
    fn read_bit(&mut self) -> Result<bool>;

    /// Read a byte without seeking forward
    fn peek_byte(&mut self) -> Result<u8>;

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
