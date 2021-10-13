#[inline(always)]
pub fn shl(value: u64, offset: u64) -> u64 {
    value.checked_shl(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn shr(value: u64, offset: u64) -> u64 {
    value.checked_shr(offset as u32).unwrap_or(0)
}

#[inline(always)]
pub fn fast_log2(value: u64) -> u64 {
    64 - value.leading_zeros() as u64
}

/// Centralized way to convert from a power of 2 to a binary mask
/// to compute the modulo fast
/// 
pub(crate) const fn power_of_two_to_mask(quantum_log2: usize) -> u64 {
    ((1_u64 << quantum_log2) - 1) as u64

    // the alternative version changes the behaviour with 
    // 0 and 64
    // `u64::MAX >> (64 - quantum_log2 as u64)`
}


#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    /// Test that we encode and decode low bits properly.
    fn test_power_of_two_to_mask() {
        assert_eq!(power_of_two_to_mask(00), 0x0000000000000000);
        assert_ne!(power_of_two_to_mask(00), 0xffffffffffffffff);
        assert_eq!(power_of_two_to_mask(64), 0x0000000000000000);
        assert_ne!(power_of_two_to_mask(64), 0xffffffffffffffff);
        
        assert_eq!(power_of_two_to_mask(01), 0x0000000000000001);
        assert_eq!(power_of_two_to_mask(02), 0x0000000000000003);
        assert_eq!(power_of_two_to_mask(03), 0x0000000000000007);
        assert_eq!(power_of_two_to_mask(04), 0x000000000000000f);
        assert_eq!(power_of_two_to_mask(05), 0x000000000000001f);
        assert_eq!(power_of_two_to_mask(06), 0x000000000000003f);
        assert_eq!(power_of_two_to_mask(07), 0x000000000000007f);
        assert_eq!(power_of_two_to_mask(08), 0x00000000000000ff);
        assert_eq!(power_of_two_to_mask(09), 0x00000000000001ff);
        assert_eq!(power_of_two_to_mask(10), 0x00000000000003ff);
        assert_eq!(power_of_two_to_mask(11), 0x00000000000007ff);
        assert_eq!(power_of_two_to_mask(12), 0x0000000000000fff);
        assert_eq!(power_of_two_to_mask(13), 0x0000000000001fff);
        assert_eq!(power_of_two_to_mask(14), 0x0000000000003fff);
        assert_eq!(power_of_two_to_mask(15), 0x0000000000007fff);
        assert_eq!(power_of_two_to_mask(16), 0x000000000000ffff);
        assert_eq!(power_of_two_to_mask(17), 0x000000000001ffff);
        assert_eq!(power_of_two_to_mask(18), 0x000000000003ffff);
        assert_eq!(power_of_two_to_mask(19), 0x000000000007ffff);
        assert_eq!(power_of_two_to_mask(20), 0x00000000000fffff);
        assert_eq!(power_of_two_to_mask(21), 0x00000000001fffff);
        assert_eq!(power_of_two_to_mask(22), 0x00000000003fffff);
        assert_eq!(power_of_two_to_mask(23), 0x00000000007fffff);
        assert_eq!(power_of_two_to_mask(24), 0x0000000000ffffff);
        assert_eq!(power_of_two_to_mask(25), 0x0000000001ffffff);
        assert_eq!(power_of_two_to_mask(26), 0x0000000003ffffff);
        assert_eq!(power_of_two_to_mask(27), 0x0000000007ffffff);
        assert_eq!(power_of_two_to_mask(28), 0x000000000fffffff);
        assert_eq!(power_of_two_to_mask(29), 0x000000001fffffff);
        assert_eq!(power_of_two_to_mask(30), 0x000000003fffffff);
        assert_eq!(power_of_two_to_mask(31), 0x000000007fffffff);
        assert_eq!(power_of_two_to_mask(32), 0x00000000ffffffff);
        assert_eq!(power_of_two_to_mask(33), 0x00000001ffffffff);
        assert_eq!(power_of_two_to_mask(34), 0x00000003ffffffff);
        assert_eq!(power_of_two_to_mask(35), 0x00000007ffffffff);
        assert_eq!(power_of_two_to_mask(36), 0x0000000fffffffff);
        assert_eq!(power_of_two_to_mask(37), 0x0000001fffffffff);
        assert_eq!(power_of_two_to_mask(38), 0x0000003fffffffff);
        assert_eq!(power_of_two_to_mask(39), 0x0000007fffffffff);
        assert_eq!(power_of_two_to_mask(40), 0x000000ffffffffff);
        assert_eq!(power_of_two_to_mask(41), 0x000001ffffffffff);
        assert_eq!(power_of_two_to_mask(42), 0x000003ffffffffff);
        assert_eq!(power_of_two_to_mask(43), 0x000007ffffffffff);
        assert_eq!(power_of_two_to_mask(44), 0x00000fffffffffff);
        assert_eq!(power_of_two_to_mask(45), 0x00001fffffffffff);
        assert_eq!(power_of_two_to_mask(46), 0x00003fffffffffff);
        assert_eq!(power_of_two_to_mask(47), 0x00007fffffffffff);
        assert_eq!(power_of_two_to_mask(48), 0x0000ffffffffffff);
        assert_eq!(power_of_two_to_mask(49), 0x0001ffffffffffff);
        assert_eq!(power_of_two_to_mask(50), 0x0003ffffffffffff);
        assert_eq!(power_of_two_to_mask(51), 0x0007ffffffffffff);
        assert_eq!(power_of_two_to_mask(52), 0x000fffffffffffff);
        assert_eq!(power_of_two_to_mask(53), 0x001fffffffffffff);
        assert_eq!(power_of_two_to_mask(54), 0x003fffffffffffff);
        assert_eq!(power_of_two_to_mask(55), 0x007fffffffffffff);
        assert_eq!(power_of_two_to_mask(56), 0x00ffffffffffffff);
        assert_eq!(power_of_two_to_mask(57), 0x01ffffffffffffff);
        assert_eq!(power_of_two_to_mask(58), 0x03ffffffffffffff);
        assert_eq!(power_of_two_to_mask(59), 0x07ffffffffffffff);
        assert_eq!(power_of_two_to_mask(60), 0x0fffffffffffffff);
        assert_eq!(power_of_two_to_mask(61), 0x1fffffffffffffff);
        assert_eq!(power_of_two_to_mask(62), 0x3fffffffffffffff);
        assert_eq!(power_of_two_to_mask(63), 0x7fffffffffffffff);
    }

    #[test]
    fn test_fast_log2() {
        assert_eq!(fast_log2(0x0000000000000000), 00);
        assert_eq!(fast_log2(0x0000000000000001), 01);
        assert_eq!(fast_log2(0x0000000000000002), 02);
        assert_eq!(fast_log2(0x0000000000000004), 03);
        assert_eq!(fast_log2(0x0000000000000008), 04);
        assert_eq!(fast_log2(0x0000000000000010), 05);
        assert_eq!(fast_log2(0x0000000000000020), 06);
        assert_eq!(fast_log2(0x0000000000000040), 07);
        assert_eq!(fast_log2(0x0000000000000080), 08);
        assert_eq!(fast_log2(0x0000000000000100), 09);
        assert_eq!(fast_log2(0x0000000000000200), 10);
        assert_eq!(fast_log2(0x0000000000000400), 11);
        assert_eq!(fast_log2(0x0000000000000800), 12);
        assert_eq!(fast_log2(0x0000000000001000), 13);
        assert_eq!(fast_log2(0x0000000000002000), 14);
        assert_eq!(fast_log2(0x0000000000004000), 15);
        assert_eq!(fast_log2(0x0000000000008000), 16);
        assert_eq!(fast_log2(0x0000000000010000), 17);
        assert_eq!(fast_log2(0x0000000000020000), 18);
        assert_eq!(fast_log2(0x0000000000040000), 19);
        assert_eq!(fast_log2(0x0000000000080000), 20);
        assert_eq!(fast_log2(0x0000000000100000), 21);
        assert_eq!(fast_log2(0x0000000000200000), 22);
        assert_eq!(fast_log2(0x0000000000400000), 23);
        assert_eq!(fast_log2(0x0000000000800000), 24);
        assert_eq!(fast_log2(0x0000000001000000), 25);
        assert_eq!(fast_log2(0x0000000002000000), 26);
        assert_eq!(fast_log2(0x0000000004000000), 27);
        assert_eq!(fast_log2(0x0000000008000000), 28);
        assert_eq!(fast_log2(0x0000000010000000), 29);
        assert_eq!(fast_log2(0x0000000020000000), 30);
        assert_eq!(fast_log2(0x0000000040000000), 31);
        assert_eq!(fast_log2(0x0000000080000000), 32);
        assert_eq!(fast_log2(0x0000000100000000), 33);
        assert_eq!(fast_log2(0x0000000200000000), 34);
        assert_eq!(fast_log2(0x0000000400000000), 35);
        assert_eq!(fast_log2(0x0000000800000000), 36);
        assert_eq!(fast_log2(0x0000001000000000), 37);
        assert_eq!(fast_log2(0x0000002000000000), 38);
        assert_eq!(fast_log2(0x0000004000000000), 39);
        assert_eq!(fast_log2(0x0000008000000000), 40);
        assert_eq!(fast_log2(0x0000010000000000), 41);
        assert_eq!(fast_log2(0x0000020000000000), 42);
        assert_eq!(fast_log2(0x0000040000000000), 43);
        assert_eq!(fast_log2(0x0000080000000000), 44);
        assert_eq!(fast_log2(0x0000100000000000), 45);
        assert_eq!(fast_log2(0x0000200000000000), 46);
        assert_eq!(fast_log2(0x0000400000000000), 47);
        assert_eq!(fast_log2(0x0000800000000000), 48);
        assert_eq!(fast_log2(0x0001000000000000), 49);
        assert_eq!(fast_log2(0x0002000000000000), 50);
        assert_eq!(fast_log2(0x0004000000000000), 51);
        assert_eq!(fast_log2(0x0008000000000000), 52);
        assert_eq!(fast_log2(0x0010000000000000), 53);
        assert_eq!(fast_log2(0x0020000000000000), 54);
        assert_eq!(fast_log2(0x0040000000000000), 55);
        assert_eq!(fast_log2(0x0080000000000000), 56);
        assert_eq!(fast_log2(0x0100000000000000), 57);
        assert_eq!(fast_log2(0x0200000000000000), 58);
        assert_eq!(fast_log2(0x0400000000000000), 59);
        assert_eq!(fast_log2(0x0800000000000000), 60);
        assert_eq!(fast_log2(0x1000000000000000), 61);
        assert_eq!(fast_log2(0x2000000000000000), 62);
        assert_eq!(fast_log2(0x4000000000000000), 63);
        assert_eq!(fast_log2(0x8000000000000000), 64);
    }
}