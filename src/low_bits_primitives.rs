use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Return the size needed to allcoate the choosen number of bits
/// This is a bit bigger than the minimum amount because doing so allows
/// us to always perform operations in a branchless way.
/// so we pay 3 extra words of memory to speed up the operations which is usually
/// worth because it would be an overhead of 24 bytes over the vector which could
/// occupy GiB of memory.
pub fn get_vec_size(n_bits: u64, size: usize) -> u64 {
    3 + ((size as u64 * n_bits) >> WORD_SHIFT)
}

#[inline(always)]
pub fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

pub fn safe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = (pos >> WORD_SHIFT) as usize;
    array[base] |= lower;
    array[base + 1] |= higher;
}

pub fn concurrent_write(array: &Vec<AtomicU64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = (pos >> WORD_SHIFT) as usize;
    array[base].fetch_or(lower, Ordering::SeqCst);
    array[base + 1].fetch_or(higher, Ordering::SeqCst);
}

#[inline(always)]
pub fn safe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = (pos >> WORD_SHIFT) as usize;
    let lower = shr(array[base], o1) & mask;
    let higher = shl(array[base + 1], o2);

    (higher | lower) & mask
}

#[inline(always)]
pub fn unsafe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let base = (pos >> WORD_SHIFT) as usize;
    let lower = shl(value, o1);
    let higher = shr(value, o2);

    unsafe {
        *array.get_unchecked_mut(base) |= lower;
        *array.get_unchecked_mut(base + 1) |= higher;
    }
}

#[inline(always)]
pub fn unsafe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = (pos >> WORD_SHIFT) as usize;
    unsafe {
        let lower = shr(*array.get_unchecked(base), o1) & mask;
        let higher = shl(*array.get_unchecked(base + 1), o2);
        (higher | lower) & mask
    }
}
