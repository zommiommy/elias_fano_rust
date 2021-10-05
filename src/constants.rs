use std::mem::size_of;


// Vigna uses 256 
// I think 1024 could be a better trade-off
// and the best memory wise would be 16'384
// because it would index in which memory page
// each value is, thus exploiting at best the 
// TLB.
pub const INDEX_SHIFT: u64 = 11;
pub const INDEX_MASK:  u64 = (1 << INDEX_SHIFT) - 1;

pub const WORD_SHIFT: u64 = 6; // log2(WORD_SIZE)
pub const WORD_SIZE: u64 = (8 * size_of::<u64>()) as u64;
pub const WORD_MASK: u64 = (1 << WORD_SHIFT) - 1;