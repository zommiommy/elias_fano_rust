#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate rand;
use elias_fano_rust::*;

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_fast_log2_floor(b: &mut Bencher) {
    let x = 1023;
    b.iter(|| utils::fast_log2_floor(black_box(x)))
}

#[bench]
fn bench_log2_floor(b: &mut Bencher) {
    let x = 1023;
    b.iter(|| (black_box(x) as f64).log2().floor() as u64)
}

#[bench]
fn bench_fast_log2_ceil(b: &mut Bencher) {
    let x = 1023;
    b.iter(|| utils::fast_log2_ceil(black_box(x)))
}

#[bench]
fn bench_log2_ceil(b: &mut Bencher) {
    let x = 1023;
    b.iter(|| (black_box(x) as f64).log2().ceil() as u64)
}
