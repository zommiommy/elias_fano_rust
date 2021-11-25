use elias_fano_rust::elias_fano::*;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;

pub const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe, 0xde, 0xad, 0xbe, 0xef, 0xc0, 0xfe, 0xbe, 0xbe,
];

/// Test that everything runs properly in the PPI graph.
pub fn build_random_sorted_vector(size: usize, max: u64) -> Vec<usize> {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut vector = Vec::new();
    for _ in 0..size {
        let t = (rng.next_u64() % max) as usize;
        vector.push(t);
    }
    vector.sort_unstable();
    vector
}

const SIZE: usize = 23_751_196;
const MAX: u64 = 462_304 * 462_304;

#[cfg(feature = "par_iter")]
#[test]
/// Test that we can build successfully run all methods in elias fano.
pub fn test_concurrent_builder() {
    // gen a random vector of values
    println!("Building vec");
    let start = Instant::now();
    let vector = build_random_sorted_vector(SIZE, MAX);
    assert_eq!(vector.len(), SIZE);
    println!("Done: {} s\n", start.elapsed().as_secs_f64());

    println!("Concurrent Builder");
    let start = Instant::now();
    // build the elias-fano concurrently
    let builder = ConcurrentEliasFanoBuilder::<10>::new(SIZE as u64, MAX).unwrap();
    vector.par_iter().enumerate().for_each(|(i, v)| {
        builder.set(i as u64, *v);
    });
    let mut ef = builder.build().unwrap();
    println!("Done: {} s", start.elapsed().as_secs_f64());
    ef.shrink_to_fit();
    println!(
        "Total size = {:.5} Mib",
        ef.size() as f64 / (1024 * 1024) as f64
    );
    println!(
        "Overhead          = {:.5} {:.5} Mib",
        ef.overhead_ratio(),
        ef.overhead() as f64 / (1024 * 1024) as f64
    );
    println!("Overhead Highbits = {:.5}", ef.overhead_high_bits_ratio());
    println!("{:#4?}", ef.memory_stats());

    println!("Sequential builder");
    let start = Instant::now();
    // build and hash the elias fano sequentially
    let mut seq = EliasFano::<10>::from_vec(&vector).unwrap();
    let mut hasher = DefaultHasher::new();
    seq.hash(&mut hasher);
    println!("Done: {} s", start.elapsed().as_secs_f64());
    let seq_hash = hasher.finish();
    seq.shrink_to_fit();
    println!(
        "Total size = {:.5} Mib",
        seq.size() as f64 / (1024 * 1024) as f64
    );
    println!(
        "Overhead          = {:.5} {:.5} Mib",
        seq.overhead_ratio(),
        seq.overhead() as f64 / (1024 * 1024) as f64
    );
    println!("Overhead Highbits = {:.5}", seq.overhead_high_bits_ratio());
    println!("{:#4?}", seq.memory_stats());

    // hash it
    let mut hasher = DefaultHasher::new();
    ef.hash(&mut hasher);
    let concurrent_hash = hasher.finish();

    // the hashses should be equal!!
    assert_eq!(seq_hash, concurrent_hash);
    assert_eq!(ef.high_bits.high_bits, seq.high_bits.high_bits);

    assert_eq!(
        ef.high_bits.high_bits_index_ones,
        seq.high_bits.high_bits_index_ones
    );
    assert_eq!(
        ef.high_bits.high_bits_index_zeros,
        seq.high_bits.high_bits_index_zeros
    );

    for i in 0..SIZE {
        assert_eq!(vector[i], seq.unchecked_select(i as u64));
        assert_eq!(vector[i], ef.unchecked_select(i as u64));
    }
}
