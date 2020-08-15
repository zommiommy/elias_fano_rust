pub const WORD_SIZE: u64 = 64;

#[inline(always)]
pub fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

use core::arch::x86_64::{
    _popcnt64,
    _pdep_u64,
    _tzcnt_u64
};

pub fn popcnt(value: u64) -> u64 {
    unsafe{_popcnt64(value as i64) as u64}
}

pub fn pdep(value: u64, position: u64) -> u64 {
    unsafe{_pdep_u64(value, position) as u64}
}

pub fn tzcnt(value: u64) -> u64 {
    unsafe{_tzcnt_u64(value) as u64}
}
