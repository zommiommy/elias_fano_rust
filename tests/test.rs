use elias_fano_rust::EliasFano;

use rand::{Rng, SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;

const TRIALS: u64 = 1_000;
const MAX: u64 = 10_000;

const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe,
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe   
];


#[test]
/// Test that everything runs properly in the PPI graph.
fn test_build() {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut vector = Vec::new();
    for _ in 0..TRIALS {
        let t = rng.next_u64() % MAX;
        vector.push(t);
    }
    vector.sort();

    let max = vector[vector.len() - 1];
    let ef = EliasFano::new(vector.clone(), max);

    for (i, v) in vector.iter().enumerate() {
        println!("Select: {} {}", *v, ef.select(i as u64).unwrap());
        assert_eq!(*v, ef.select(i as u64).unwrap());
        println!("Rank: {} {}", i as u64, ef.rank(*v).unwrap());
        assert_eq!(i as u64, ef.rank(*v).unwrap());
    }
}
