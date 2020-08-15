use super::*;

#[inline(always)]
pub fn population_count(word: u64) -> u64 {
    popcnt(word)
}

#[inline(always)]
pub fn index_of_nth_one_in_word(word: u64, index:u64) -> u64 {
    tzcnt(pdep(1 << index, word))
}

#[inline(always)]
pub fn index_of_nth_zeros_in_word(word: u64, index:u64) -> u64 {
    tzcnt(pdep(1 << index, !word))
}

#[inline(always)]
pub fn n_of_consecutive_ones(word: u64, index:u64) -> u64 {
    tzcnt(!(word >> index))
}

#[inline(always)]
pub fn n_of_consecutive_ones_from_start(word: u64) -> u64 {
    tzcnt(!word)
}