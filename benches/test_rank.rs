#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate fid;
extern crate rand;
extern crate test;

use fid::{BitVector, FID};
use rand::{Rng, SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;
use test::{Bencher, black_box};

const TRIALS: u64 = 10_000;

const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe,
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe   
];

#[bench]
fn rank_1_000_000_dense(b: &mut Bencher) {
    bench_rank1(1_000_000, 0.99, b);
}

#[bench]
fn rank_1_000_000_normal(b: &mut Bencher) {
    bench_rank1(1_000_000, 0.5, b);
}

#[bench]
fn rank_1_000_000_sparse(b: &mut Bencher) {
    bench_rank1(1_000_000, 0.01, b);
}

#[bench]
fn rank_1_000_000_vec(b: &mut Bencher) {
    bench_vector_rank(1_000_000, b);
}

#[bench]
fn rank_1_000_000_ef(b: &mut Bencher) {
    bench_ef_rank(1_000_000, b);
}

#[bench]
fn rank_100_000_000_dense(b: &mut Bencher) {
    bench_rank1(100_000_000, 0.99, b);
}

#[bench]
fn rank_100_000_000_normal(b: &mut Bencher) {
    bench_rank1(100_000_000, 0.5, b);
}

#[bench]
fn rank_100_000_000_sparse(b: &mut Bencher) {
    bench_rank1(100_000_000, 0.01, b);
}

#[bench]
fn rank_100_000_000_vec(b: &mut Bencher) {
    bench_vector_rank(100_000_000, b);
}

#[bench]
fn rank_100_000_000_ef(b: &mut Bencher) {
    bench_ef_rank(100_000_000, b);
}

fn bench_rank1(n: u64, p: f64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut bv = BitVector::new();
    for _ in 0..n {
        let b = rng.gen_bool(p);
        bv.push(b);
    }
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(bv.rank1(rng.gen_range(0, n)));
        }
    })
}

fn bench_vector_rank(n: u64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rng.next_u64());
    }
    v.sort();
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(v.binary_search(&rng.gen_range(0, u64::MAX)));
        }
    })
}

fn bench_ef_rank(n: u64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rng.next_u64());
    }
    v.sort();
    let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(ef.rank(rng.gen_range(0, n)));
        }
    })
}