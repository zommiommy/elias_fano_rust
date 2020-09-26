pub const WORD_SIZE: u64 = 64;

#[inline(always)]
pub fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}