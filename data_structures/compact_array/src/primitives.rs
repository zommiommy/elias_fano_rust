use crate::{constants::*, utils::*};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Return the size needed to allcoate the choosen number of bits
/// This is a bit bigger than the minimum amount because doing so allows
/// us to always perform operations in a branchless way.
/// so we pay 3 extra words of memory to speed up the operations which is usually
/// worth because it would be an overhead of 24 bytes over the vector which could
/// occupy GiB of memory.
pub fn get_vec_size(n_bits: usize, size: usize) -> usize {
    3 + ((size as usize * n_bits) >> WORD_SHIFT)
}

#[inline(always)]
#[allow(dead_code)]
pub fn safe_write(array: &mut Vec<usize>, index: usize, value: usize, value_size: usize) {
    let pos = index * value_size;
    let o1 = pos & WORD_BIT_SIZE_MASK;
    let o2 = WORD_BIT_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = (pos >> WORD_SHIFT) as usize;
    array[base] |= lower;
    array[base + 1] |= higher;
}

#[inline(always)]
#[allow(dead_code)]
pub fn concurrent_write(array: &Vec<AtomicUsize>, index: usize, value: usize, value_size: usize) {
    let pos = index * value_size;
    let o1 = pos & WORD_BIT_SIZE_MASK;
    let o2 = WORD_BIT_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = (pos >> WORD_SHIFT) as usize;
    array[base].fetch_or(lower, Ordering::SeqCst);
    array[base + 1].fetch_or(higher, Ordering::SeqCst);
}

#[inline(always)]
#[allow(dead_code)]
pub fn safe_read(array: &[usize], index: usize, value_size: usize) -> usize {
    let pos = index * value_size;
    let o1 = pos & WORD_BIT_SIZE_MASK;
    let o2 = WORD_BIT_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = (pos >> WORD_SHIFT) as usize;
    let lower = shr(array[base], o1) & mask;
    let higher = shl(array[base + 1], o2);

    (higher | lower) & mask
}

#[inline(always)]
#[allow(dead_code)]
pub fn unsafe_write(array: &mut Vec<usize>, index: usize, value: usize, value_size: usize) {
    let pos = index * value_size;
    let o1 = pos & WORD_BIT_SIZE_MASK;
    let o2 = WORD_BIT_SIZE - o1;

    let base = (pos >> WORD_SHIFT) as usize;
    let lower = shl(value, o1);
    let higher = shr(value, o2);

    unsafe {
        *array.get_unchecked_mut(base) |= lower;
        *array.get_unchecked_mut(base + 1) |= higher;
    }
}

#[inline(always)]
#[allow(dead_code)]
pub fn unsafe_read(array: &[usize], index: usize, value_size: usize) -> usize {
    let pos = index * value_size;
    let o1 = pos & WORD_BIT_SIZE_MASK;
    let o2 = WORD_BIT_SIZE - o1;

    let mask = (1 << value_size) - 1;
    let base = (pos >> WORD_SHIFT) as usize;
    unsafe {
        let lower = shr(*array.get_unchecked(base), o1) & mask;
        let higher = shl(*array.get_unchecked(base + 1), o2);
        (higher | lower) & mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::Rng;
    use rand::SeedableRng;

    pub const SEED: [u8; 16] = [
        0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe,
        0xbe,
    ];

    /// Test that everything runs properly in the PPI graph.
    pub fn build_random_sorted_vector(size: usize, max: usize) -> Vec<usize> {
        let mut rng: SmallRng = SmallRng::from_seed(SEED);
        let mut vector = Vec::new();
        for _ in 0..size {
            let t = rng.gen::<usize>() % max;
            vector.push(t);
        }
        vector.sort();
        vector
    }

    fn test_safe_low_bits(n_bits: usize, size: usize) {
        let max_values = (1 << n_bits) - 1;
        let mut low_bits = vec![0; get_vec_size(n_bits, size) as usize];

        let vector = build_random_sorted_vector(size, max_values);
        let values: Vec<usize> = vector.iter().map(|x| x % max_values).collect();

        for (i, v) in values.iter().enumerate() {
            safe_write(&mut low_bits, i as usize, *v, n_bits);
        }

        for (i, v) in values.iter().enumerate() {
            assert_eq!(*v, safe_read(&low_bits, i as usize, n_bits));
        }
    }

    fn test_unsafe_low_bits(n_bits: usize, size: usize) {
        let max_values = (1 << n_bits) - 1;
        let mut low_bits = vec![0; get_vec_size(n_bits, size) as usize];

        let vector = build_random_sorted_vector(size, max_values);
        let values: Vec<usize> = vector.iter().map(|x| x % max_values).collect();

        for (i, v) in values.iter().enumerate() {
            unsafe_write(&mut low_bits, i as usize, *v, n_bits);
        }

        for (i, v) in values.iter().enumerate() {
            assert_eq!(*v, unsafe_read(&low_bits, i as usize, n_bits));
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_low_bits_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..10_000 {
            test_safe_low_bits(rng.gen_range(1, 64), rng.gen_range(1, 1000));
            test_unsafe_low_bits(rng.gen_range(1, 64), rng.gen_range(1, 1000));
        }
    }
}
