mod utils;
use utils::*;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn grid_test() {
    (1..1_000).step_by(100).for_each(|size| {
        for max in (10..1_000).step_by(100) {
            let result = default_test_suite(size, max as u64);
            assert!(size != 0 || result.is_err());
        }
    });
}
