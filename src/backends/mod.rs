//! Collection of In-memory structs, or wrappers on which we can write and read
//! codes.
//!
//! `BitArrayLittle` and `BitArrayBig` allow to convert any indexable structure
//! to one that can use all the codes.
//!
//! `BitArrayLittle` read words from the LSB to the MSB, this is really hard to
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

mod mmap;
pub use mmap::*;

// mod bitarray_little;
// pub use bitarray_little::*;

mod bitarray_m2l;
pub use bitarray_m2l::*;

// mod file_backend;
// pub use file_backend::*;

// mod bitstream;
// pub use bitstream::*;
