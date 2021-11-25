use elias_fano_rust::elias_fano::EliasFano;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe,
];

/// Test that everything runs properly in the PPI graph.
pub fn build_random_sorted_vector(size: usize, max: usize) -> Vec<usize> {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut vector = Vec::new();
    for _ in 0..size {
        let t = rng.next_u64() as usize % max;
        vector.push(t);
    }
    vector.sort();
    vector
}

#[test]
/// Test that we can build successfully run all methods in elias fano.
pub fn test_hash() {
    let vector = build_random_sorted_vector(1_000, 1_000_000);
    let ef1 = EliasFano::<10>::from_vec(&vector).unwrap();
    let ef2 = EliasFano::<10>::from_vec(&vector).unwrap();

    let mut hasher = DefaultHasher::new();
    ef1.hash(&mut hasher);
    let h1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    ef2.hash(&mut hasher);
    let h2 = hasher.finish();

    assert_eq!(h1, h2);
}
