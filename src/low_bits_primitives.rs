use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Return the size needed to allcoate the choosen number of bits
/// This is a bit bigger than the minimum amount because doing so allows
/// us to always perform operations in a branchless way.
/// so we pay 3 extra words of memory to speed up the operations which is usually
/// worth because it would be an overhead of 24 bytes over the vector which could
/// occupy GiB of memory.
pub(crate) fn get_vec_size(n_bits: u64, size: usize) -> u64 {
    3 + ((size as u64 * n_bits) >> WORD_SHIFT)
}

#[inline(always)]
pub(crate) fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub(crate) fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

pub(crate) fn safe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
    let pos = index * value_size;
    let o1 = pos & WORD_MASK;
    let o2 = WORD_SIZE - o1;

    let lower = shl(value, o1);
    let higher = shr(value, o2);

    let base = (pos >> WORD_SHIFT) as usize;
    array[base] |= lower;
    array[base + 1] |= higher;
}

pub(crate) fn concurrent_write(array: &Vec<AtomicU64>, index: u64, value: u64, value_size: u64) {
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
pub(crate) fn safe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
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
pub(crate) fn unsafe_write(array: &mut Vec<u64>, index: u64, value: u64, value_size: u64) {
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
pub(crate) fn unsafe_read(array: &[u64], index: u64, value_size: u64) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::RngCore;
    use rand::SeedableRng;

    pub const SEED: [u8; 16] = [
        0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe,
    ];

    /// Test that everything runs properly in the PPI graph.
    pub fn build_random_sorted_vector(size: usize, max: u64) -> Vec<u64> {
        let mut rng: SmallRng = SmallRng::from_seed(SEED);
        let mut vector = Vec::new();
        for _ in 0..size {
            let t = rng.next_u64() % max;
            vector.push(t);
        }
        vector.sort();
        vector
    }


    /// Test that we can build successfully run all methods in elias fano.
    pub fn default_test_suite(size:usize, max:u64) -> Result<(), String>{
        let vector = build_random_sorted_vector(size, max);
        let ef = EliasFano::from_vec(&vector)?;
        vector.iter().enumerate().for_each(|(i, v)| {
            assert_eq!(*v, ef.select(i as u64).unwrap());
            assert!(ef.contains(*v));
            assert_eq!(*v, ef.unchecked_select(i as u64));
            assert_eq!(ef.select(ef.unchecked_rank(*v)).unwrap(), *v);
        });

        //ef.debug();

        Ok(())
    }


    fn test_safe_low_bits(n_bits: u64, size: usize){
        let max_values = (1 << n_bits) - 1;
        let mut low_bits = vec![0; get_vec_size(n_bits, size) as usize];

        
        let vector = build_random_sorted_vector(size, max_values);
        let values: Vec<u64> = vector.iter().map(|x| x % max_values).collect();
        
        for (i, v) in values.iter().enumerate() {
            safe_write(&mut low_bits, i as u64, *v, n_bits);
        }

        for (i, v) in values.iter().enumerate() {
            assert_eq!(
                *v,
                safe_read(&low_bits, i as u64, n_bits)
            );
        }
    }

    fn test_unsafe_low_bits(n_bits: u64, size: usize){
        let max_values = (1 << n_bits) - 1;
        let mut low_bits = vec![0; get_vec_size(n_bits, size) as usize];

        
        let vector = build_random_sorted_vector(size, max_values);
        let values: Vec<u64> = vector.iter().map(|x| x % max_values).collect();
        
        for (i, v) in values.iter().enumerate() {
            unsafe_write(&mut low_bits, i as u64, *v, n_bits);
        }

        for (i, v) in values.iter().enumerate() {
            assert_eq!(
                *v,
                unsafe_read(&low_bits, i as u64, n_bits)
            );
        }
    }

    use rand::Rng;

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