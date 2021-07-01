use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use elias_fano_rust::*;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use rayon::prelude::*;

pub const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe,
];

/// Test that everything runs properly in the PPI graph.
pub fn build_random_sorted_vector(size: usize, max: u64) -> Vec<u64> {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut vector = Vec::new();
    for _ in 0..size {
        let t = rng.next_u64() % max;
        vector.push(t);
    }
    vector.sort();
    vector
}


#[test]
/// Test that we can build successfully run all methods in elias fano.
pub fn test_concurrent_builder() {
    // gen a random vector of values
    let vector = build_random_sorted_vector(1_000, 1_000_000);

    // build and hash the elias fano sequentially
    let seq = EliasFano::from_vec(&vector).unwrap();
    let mut hasher = DefaultHasher::new();
    seq.hash(&mut hasher);
    let seq_hash = hasher.finish();


    // build the elias-fano concurrently
    let builder = ConcurrentEliasFanoBuilder::new(1_000, 1_000_000).unwrap();
    vector.iter().enumerate().for_each(|(i, v)| {
        builder.set(i as u64, *v);
    });
    let ef = builder.build();

    // hash it
    let mut hasher = DefaultHasher::new();
    ef.hash(&mut hasher);
    let concurrent_hash = hasher.finish();

    // the hashses should be equal!!
    assert_eq!(seq_hash, concurrent_hash);
}
