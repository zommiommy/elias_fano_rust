#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate rand;

use rand::{Rng, SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;

extern crate test;
use test::{Bencher, black_box};

const SIZE: u64 = 1_000_000;
const MAX: u64 = 2 * SIZE;

const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe,
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe   
];

pub(crate) fn test_vector() -> (Vec<u64>, SmallRng) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..SIZE {
        v.push(rng.next_u64() % MAX);
    }
    v.sort();
    (v, rng)
}

mod ef {
    use super::*;
        
    #[bench]
    fn iter_select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
        b.iter(|| {
            ef.iter_select().collect::<Vec<_>>()
        })
    }

    #[bench]
    fn iter_new(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
        b.iter(|| {
            ef.iter().collect::<Vec<_>>()
        })
    }
}
