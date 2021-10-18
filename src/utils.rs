#[inline(always)]
pub fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

#[inline(always)]
/// based on https://bugzilla.mozilla.org/show_bug.cgi?id=327129
/// 
/// This should compile to:
/// ```ignore
/// or      rdi, 1
/// lzcnt   rax, rdi
/// xor     rax, 63
/// ```
pub const fn fast_log2_floor(value: u64) -> u64 {
    (63 - (value | 1).leading_zeros()) as u64
}

#[inline(always)]
pub const fn fast_log2_ceil(value: u64) -> u64 {
    (64 - (value.saturating_sub(1)).leading_zeros()) as u64
}

#[inline(always)]
pub const fn fast_pow_2(exp: u64) -> u64 {
    1 << exp
}

/// Centralized way to convert from a power of 2 to a binary mask
/// to compute the modulo fast
/// 
pub const fn power_of_two_to_mask(quantum_log2: usize) -> u64 {
    ((1_u64 << quantum_log2) - 1) as u64

    // the alternative version changes the behaviour with 
    // 0 and 64
    // `u64::MAX >> (64 - quantum_log2 as u64)`
}


#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_fast_pow_2() {
        for i in 0..63 {
            assert_eq!(2_u64.pow(i), fast_pow_2(i as _));
        }
    }

    #[test]
    fn test_fast_log2_ceil() {
        for i in 0..(1 << 16) {
            assert_eq!((i as f64).log2().ceil() as u64, fast_log2_ceil(i));
        }
    }

    #[test]
    fn test_fast_log2_floor() {
        for i in 0..(1 << 16) {
            assert_eq!((i as f64).log2().floor() as u64, fast_log2_floor(i), "{}", i);
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