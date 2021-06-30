// HACK: To prevent rust from warning about the unused unsafe block in debug builds
#[allow(dead_code)]
#[inline(always)]
pub(crate) const unsafe fn unsafe_noop_fn() {}

#[macro_export]
/// Macro to instruct the compiler that a value is guaranteed to be in a certain range
/// https://www.reddit.com/r/rust/comments/mr08rc/optimize_code_by_hinting_at_range_of_value/
macro_rules! hint_in_range {
    ($lo:literal.. $hi:literal, $val:expr) => {
        #[allow(unused_comparisons)]
        if $val < $lo || $val >= $hi {
            #[cfg(not(debug_assertions))]
            std::hint::unreachable_unchecked();

            #[cfg(debug_assertions)]
            {
                unsafe_noop_fn();
                panic!("value: {} out of range: {}..{}", $val, $lo, $hi);
            }
        }
    };
    ($lo:literal..= $hi:literal, $val:expr) => {
        #[allow(unused_comparisons)]
        if $val < $lo || $val > $hi {
            #[cfg(not(debug_assertions))]
            std::hint::unreachable_unchecked();

            #[cfg(debug_assertions)]
            {
                unsafe_noop_fn();
                panic!("value: {} out of range: {}..={}", $val, $lo, $hi);
            }
        }
    };
}