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
fn select_1_000_000_dense(b: &mut Bencher) {
    bench_select1(1_000_000, 0.99, b);
}

#[bench]
fn select_1_000_000_normal(b: &mut Bencher) {
    bench_select1(1_000_000, 0.5, b);
}

#[bench]
fn select_1_000_000_sparse(b: &mut Bencher) {
    bench_select1(1_000_000, 0.01, b);
}

#[bench]
fn select_1_000_000_vec(b: &mut Bencher) {
    bench_vector_select(1_000_000, b);
}

#[bench]
fn select_1_000_000_ef(b: &mut Bencher) {
    bench_ef_select(1_000_000, b);
}

#[bench]
fn select_100_000_000_dense(b: &mut Bencher) {
    bench_select1(100_000_000, 0.99, b);
}

#[bench]
fn select_100_000_000_normal(b: &mut Bencher) {
    bench_select1(100_000_000, 0.5, b);
}

#[bench]
fn select_100_000_000_sparse(b: &mut Bencher) {
    bench_select1(100_000_000, 0.01, b);
}

#[bench]
fn select_100_000_000_vec(b: &mut Bencher) {
    bench_vector_select(100_000_000, b);
}

#[bench]
fn select_100_000_000_ef(b: &mut Bencher) {
    bench_ef_select(100_000_000, b);
}


fn bench_select1(n: u64, p: f64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut bv = BitVector::new();
    let mut rank = 0;
    for _ in 0..n {
        let b = rng.gen_bool(p);
        bv.push(b);
        rank += b as u64;
    }
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(bv.select1(rng.gen_range(0, rank)));
        }
    })
}


fn bench_vector_select(n: u64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rng.next_u64());
    }
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(v[rng.gen_range(0, n) as usize]);
        }
    })
}

fn bench_ef_select(n: u64, b: &mut Bencher) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rng.next_u64());
    }
    v.sort();
    let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
    b.iter(|| {
        for _ in 0..TRIALS {
            black_box(ef.select(rng.gen_range(0, n)));
        }
    })
}
