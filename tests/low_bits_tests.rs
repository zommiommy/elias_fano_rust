use elias_fano_rust::*;

mod utils;
use utils::*;

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
