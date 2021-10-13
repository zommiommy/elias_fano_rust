pub const WORD_SHIFT: u64 = 6; // log2(WORD_SIZE)
pub const WORD_SIZE: u64 = (8 * std::mem::size_of::<u64>()) as u64;
pub const WORD_MASK: u64 = (1 << WORD_SHIFT) - 1;