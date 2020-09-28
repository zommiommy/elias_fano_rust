use elias_fano_rust::EliasFano;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
mod utils;
use utils::*;

/// Test that we can build successfully run all methods in elias fano.
fn default_test_suite(size:usize, max:u64) -> Result<(), String>{
    let vector = build_random_sorted_vector(size, max);
    let ef = EliasFano::from_vec(&vector)?;
    vector.iter().enumerate().for_each(|(i, v)| {
        assert_eq!(*v, ef.select(i as u64).unwrap());
        assert!(ef.contains(*v));
        assert_eq!(*v, ef.unchecked_select(i as u64));
        assert_eq!(ef.select(ef.rank(*v)).unwrap(), *v);
    });

    ef.debug();

    Ok(())
}

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_with_fuzzing() {
    (0..1_000).step_by(100).par_bridge().for_each(|size| {
        for max in (10..1_000).step_by(100){
            let result = default_test_suite(size, max as u64);
            assert!(size!=0 || result.is_err());            
        }
    });
}