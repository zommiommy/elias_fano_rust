//! Collection of fast utilities that are used in all the crate

use crate::constants::*;

#[inline(always)]
/// Shift left x86_64 instruction
pub fn shl(value: usize, offset: usize) -> usize {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
/// Shift right x86_64 instruction
pub fn shr(value: usize, offset: usize) -> usize {
    value.checked_shr(offset as u32).unwrap_or(0)
}

#[inline(always)]
/// based on <https://bugzilla.mozilla.org/show_bug.cgi?id=327129>
///
/// On `x86_64` this should compile to:
/// ```asm
/// or      rdi, 1
/// lzcnt   rax, rdi
/// xor     rax, 63
/// ```
/// or
/// ```asm
/// or      rdi, 1
/// bsr     rax, rdi
/// ```
pub const fn fast_log2_floor(value: usize) -> usize {
    WORD_SIZE_MINUS_ONE - (value | 1).leading_zeros() as usize
}

#[inline(always)]
pub const fn fast_log2_ceil(value: usize) -> usize {
    let base = fast_log2_floor(value); 
    let offset = if value & (value.wrapping_sub(1)) != 0 {1} else {0};
    base + offset
}

#[inline(always)]
/// Convert an exponent to a power of two
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

/// Bijective mapping from isize to usize as defined in [https://github.com/vigna/dsiutils/blob/master/src/it/unimi/dsi/bits/Fast.java]
pub const fn int2nat(x: isize) -> usize {
    if x >= 0 {
        (x as usize) << 1
    } else {
        ((-x as usize) << 1) - 1
    }

    // interpret the isize bits as an usize
    // let x = usize::from_ne_bytes(x.to_ne_bytes());
    // x << 1 ^ (x >> WORD_SIZE_MINUS_ONE)
}

/// Bijective mapping from usize to isize as defined in [https://github.com/vigna/dsiutils/blob/master/src/it/unimi/dsi/bits/Fast.java]
pub const fn nat2int(x: usize) -> isize {
    if x & 1 == 0 {
        (x >> 1) as isize
    } else {
        -(((x + 1) >> 1) as isize)
    }

    //let result = (x >> 1) ^ !((x & 1) - 1);
    //isize::from_ne_bytes(result.to_ne_bytes())
}

#[cfg(test)]
mod test_utils {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_nat2int_int2nat() {
        for i in 0..100_000 {
            assert_eq!(i, int2nat(nat2int(i)));
        }
    }

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
            assert_eq!(
                (i as f64).log2().floor() as usize,
                fast_log2_floor(i),
                "{}",
                i
            );
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
