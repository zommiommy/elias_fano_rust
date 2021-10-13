use elias_fano_rust::elias_fano::EliasFano;
use rayon::prelude::*;
mod utils;
use utils::*;

/// Test that we can build successfully run all methods in elias fano.
fn default_test_suite(size:usize, max:u64) -> Result<(), String>{
    let vector = build_random_sorted_vector(size, max);
    let true_max = *vector.last().unwrap();
    let ef = EliasFano::<10>::from_vec(&vector)?;
    (0..max).for_each(|i| {
        let index = ef.unchecked_rank(i);
        if i <= true_max {
            assert!(ef.select(index).unwrap() >= i);
        } else {
            assert!(index == vector.len() as u64);
        }
    });

    Ok(())
}

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_extended_fuzzing() {
    (0..10).into_par_iter().for_each(|size| {
        let size:usize = 1 + (size as usize)*100;
        for max in (size..1_000).step_by(100){
            let result = default_test_suite(size, max as u64);
            assert!(size!=0 || result.is_err());            
        }
    });
}