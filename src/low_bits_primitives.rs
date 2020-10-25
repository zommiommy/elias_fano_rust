use std::mem::size_of;

const WORD_SIZE: usize = 8 * size_of::<u64>();
const WORD_SHIFT: usize = 6; // log2(WORD_SIZE)
const WORD_MASK: usize = (1 << WORD_SHIFT) - 1;

pub fn get_vec_size(n_bits: u64, size: usize) -> u64 {
    3 + ((size as u64 * n_bits) >> WORD_SHIFT)
}

#[inline(always)]
pub fn shl(value: u64, offset: usize) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: usize) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

pub fn safe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = (index * value_size) as usize ;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = pos >> WORD_SHIFT;
    array[base] |= lower;
    array[base + 1] |= higher;
}

#[inline(always)]
pub fn safe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
    let pos = (index * value_size) as usize ;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = pos >> WORD_SHIFT;
    let lower = shr(array[base], o1) & mask;
    let higher = shl(array[base + 1], o2);

    (higher | lower) & mask
}

#[inline(always)]
pub fn unsafe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = (index * value_size) as usize ;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let base = pos >> WORD_SHIFT;
    let lower = shl(value, o1);
    let higher = shr(value, o2);

    unsafe {
        *array.get_unchecked_mut(base) |= lower;
        *array.get_unchecked_mut(base + 1) |= higher;
    }
}

#[inline(always)]
pub fn unsafe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
    let pos = (index * value_size) as usize ;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = pos >> WORD_SHIFT;
    unsafe {
        let lower = shr(*array.get_unchecked(base), o1) & mask;
        let higher = shl(*array.get_unchecked(base + 1), o2);
        (higher | lower) & mask
    }
}
