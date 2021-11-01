#[inline(always)]
pub fn shl(value: usize, offset: usize) -> usize {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: usize, offset: usize) -> usize {
    value.checked_shr(offset as u32).unwrap_or(0)
}

#[inline(always)]
/// based on https://bugzilla.mozilla.org/show_bug.cgi?id=327129
/// 
/// On `x86_64` this should compile to:
/// ```asm
/// or      rdi, 1
/// lzcnt   rax, rdi
/// xor     rax, 63
/// ```
pub const fn fast_log2_floor(value: usize) -> usize {
    const WORD_SIZE_MINUS_ONE: usize = 8 * core::mem::size_of::<usize>() - 1;
    WORD_SIZE_MINUS_ONE - (value | 1).leading_zeros() as usize
}

#[inline(always)]
pub const fn fast_log2_ceil(value: usize) -> usize {
    const WORD_SIZE: usize = 8 * core::mem::size_of::<usize>();
    WORD_SIZE - (value.saturating_sub(1)).leading_zeros() as usize
}

#[inline(always)]
pub const fn fast_pow_2(exp: usize) -> usize {
    1 << exp
}

#[inline(always)]
/// Centralized way to convert from a power of 2 to a binary mask
/// to compute the modulo fast
/// 
pub const fn power_of_two_to_mask(quantum_log2: usize) -> usize {
    ((1_usize << quantum_log2) - 1) as usize

    // the alternative version changes the behaviour with 
    // 0 and WORD_BIT_SIZE
    // `usize::MAX >> (WORD_BIT_SIZE - quantum_log2 as usize)`
}


#[cfg(test)]
mod test_utils {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_fast_pow_2() {
        for i in 0..WORD_BIT_SIZE {
            assert_eq!(2_usize.pow(i as u32), fast_pow_2(i as _));
        }
    }

    #[test]
    fn test_fast_log2_ceil() {
        for i in 0..(1 << 16) {
            assert_eq!((i as f64).log2().ceil() as usize, fast_log2_ceil(i));
        }
    }

    #[test]
    fn test_fast_log2_floor() {
        for i in 0..(1 << 16) {
            assert_eq!((i as f64).log2().floor() as usize, fast_log2_floor(i), "{}", i);
        }
    }

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_power_of_two_to_mask() {
        for i in 0..(1 << 16) {
            for j in 0..10 {
                assert_eq!(i % (1 << j), i & power_of_two_to_mask(j), "{}", i);
            }
        }
    }
}