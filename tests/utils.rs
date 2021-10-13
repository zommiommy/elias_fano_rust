use elias_fano_rust::elias_fano::EliasFano;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

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

#[allow(dead_code)]
/// Test that we can build successfully run all methods in elias fano.
pub fn default_test_suite(size:usize, max:u64) -> Result<(), String>{
    let vector = build_random_sorted_vector(size, max);
    let ef = EliasFano::<10>::from_vec(&vector)?;
    vector.iter().enumerate().for_each(|(i, v)| {
        assert_eq!(*v, ef.select(i as u64).unwrap());
        assert!(ef.contains(*v));
        assert_eq!(*v, ef.unchecked_select(i as u64));
        assert_eq!(ef.select(ef.unchecked_rank(*v)).unwrap(), *v);
    });

    //ef.debug();

    Ok(())
}
