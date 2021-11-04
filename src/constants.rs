use crate::utils::fast_log2_ceil;
use core::mem::size_of;
pub(crate) const WORD_BYTE_SIZE: usize = size_of::<usize>(); 
pub(crate) const WORD_BIT_SIZE: usize = 8 * WORD_BYTE_SIZE;
pub(crate) const WORD_SHIFT: usize = fast_log2_ceil(WORD_BIT_SIZE);
pub(crate) const WORD_BIT_SIZE_MASK: usize = WORD_BIT_SIZE - 1;
pub(crate) const WORD_HIGHEST_BIT_MASK: usize = 1 << (WORD_BIT_SIZE - 1);
