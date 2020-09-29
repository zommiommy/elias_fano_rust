use super::*;
use std::mem::size_of;

const WORD_SIZE: u64 = 8 * size_of::<u64>() as u64;

pub fn get_vec_size(n_bits: u64, size: usize) -> u64 {
    2 + ((size as u64 * n_bits) as f64  / WORD_SIZE as f64).ceil() as u64
}

#[inline(always)]
pub fn safe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let base = pos as usize / WORD_SIZE as usize;
    let o1 = pos % WORD_SIZE;
    let o2 = WORD_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    array[base] |= lower;
    array[base + 1] |= higher;
}

#[inline(always)]
pub fn safe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
    let pos = index * value_size;
    let base = pos as usize / WORD_SIZE as usize;
    let o1 = pos % WORD_SIZE;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let lower = shr(array[base], o1) & mask;
    let higher = shl(array[base + 1], o2);

    (higher | lower) & mask
}

#[inline(always)]
pub fn unsafe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let base = pos as usize / WORD_SIZE as usize;
    let o1 = pos % WORD_SIZE;
    let o2 = WORD_SIZE - o1;

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
    let base = pos as usize / WORD_SIZE as usize;
    let o1 = pos % WORD_SIZE;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    unsafe {
        let lower = shr(*array.get_unchecked(base), o1) & mask;
        let higher = shl(*array.get_unchecked(base + 1), o2);
        (higher | lower) & mask
    }
}
