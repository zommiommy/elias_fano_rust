#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate rand;

use rand::{Rng, SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;

extern crate test;
use test::{Bencher, black_box};

const TRIALS: u64 = 1_000;
const SIZE: u64 = 32_000_000;
const MAX : u64 = 450_000 * 450_000;

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

macro_rules! bench_ef {
    ($module:ident, $gen:literal) => {
        mod $module {
            use super::*;
                
            #[bench]
            fn rank(b: &mut Bencher) {
                let (v, mut rng) = test_vector();
                let mut ef = elias_fano_rust::elias_fano::EliasFano::<$gen>::from_vec(&v).unwrap();
                ef.shrink_to_fit();
                println!("{:?} Mib", ef.size() as f64 / 1024.0 / 1024.0);
                b.iter(|| {
                    for _ in 0..TRIALS {
                        black_box(ef.rank(rng.gen_range(0, SIZE)));
                    }
                })
            }

            #[bench]
            fn select(b: &mut Bencher) {
                let (v, mut rng) = test_vector();
                let ef = elias_fano_rust::elias_fano::EliasFano::<$gen>::from_vec(&v).unwrap();
                b.iter(|| {
                    for _ in 0..TRIALS {
                        black_box(ef.select(rng.gen_range(0, SIZE)).unwrap());
                    }
                })
            }
        }
    };
}

bench_ef!(ef_06, 6);
bench_ef!(ef_07, 7);
bench_ef!(ef_08, 8);
bench_ef!(ef_09, 9);
bench_ef!(ef_10, 10);
bench_ef!(ef_11, 11);
bench_ef!(ef_12, 12);
bench_ef!(ef_13, 13);
bench_ef!(ef_14, 14);
bench_ef!(ef_15, 15);
bench_ef!(ef_16, 16);

macro_rules! bench_si {
    ($module:ident, $gen:literal) => {
        mod $module {
            use super::*;
                
            #[bench]
            fn rank(b: &mut Bencher) {
                let (v, mut rng) = test_vector();
                let mut ss = elias_fano_rust::sparse_index::SparseIndex::<$gen>::from_vec(v);
                ss.shrink_to_fit();
                println!("{} Mib", ss.size().total() as f64 / 1024.0 / 1024.0);
                b.iter(|| {
                    for _ in 0..TRIALS {
                        black_box(ss.rank1(rng.gen_range(0, SIZE)));
                    }
                })
            }

            #[bench]
            fn select(b: &mut Bencher) {
                let (v, mut rng) = test_vector();
                let ss = elias_fano_rust::sparse_index::SparseIndex::<$gen>::from_vec(v);
                b.iter(|| {
                    for _ in 0..TRIALS {
                        black_box(ss.select1(rng.gen_range(0, SIZE)));
                    }
                })
            }
        }
    };
}

bench_si!(si_06, 6);
bench_si!(si_07, 7);
bench_si!(si_08, 8);
bench_si!(si_09, 9);
bench_si!(si_10, 10);
bench_si!(si_11, 11);
bench_si!(si_12, 12);
bench_si!(si_13, 13);
bench_si!(si_14, 14);
bench_si!(si_15, 15);
bench_si!(si_16, 16);
