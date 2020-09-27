use elias_fano_rust::EliasFano;
use rayon::prelude::*;
mod utils;
use utils::*;

/// Test that we can build successfully run all methods in elias fano.
fn default_test_suite(size:usize, max:u64) -> Result<(), String>{
    let vector = build_random_sorted_vector(size, max);
    let ef = EliasFano::from_vec(&vector)?;
    vector.iter().enumerate().for_each(|(i, v)| {
        assert_eq!(*v, ef.select(i as u64).unwrap());
        assert_eq!(*v, ef.unchecked_select(i as u64));
        assert_eq!(ef.select(ef.rank(*v).unwrap()).unwrap(), *v);
        assert_eq!(ef.unchecked_select(ef.unchecked_rank(*v)), *v);
    });

    Ok(())
}

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_with_fuzzing() {
    (0..10).into_par_iter().for_each(|size| {
        let size:usize = (size as usize)*100;
        for max in (size..1_000).step_by(100){
            let result = default_test_suite(size, max as u64);
            assert!(size!=0 || result.is_err());            
        }
    });
}