use std::mem::size_of;


/// Vigna uses 8, but in our experiments 10 or 11 provide better trade-offs.
/// more info in the crate documentation.
pub const INDEX_SHIFT: u64 = 10;
pub const INDEX_MASK:  u64 = (1 << INDEX_SHIFT) - 1;

pub const WORD_SHIFT: u64 = 6; // log2(WORD_SIZE)
pub const WORD_SIZE: u64 = (8 * size_of::<u64>()) as u64;
pub const WORD_MASK: u64 = (1 << WORD_SHIFT) - 1;