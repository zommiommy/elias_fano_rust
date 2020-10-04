#![feature(asm)]
#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

extern crate elias_fano_rust;
use elias_fano_rust::*;
use vec_rand::gen_random_vec;

const ARRAY_SIZE: usize = 1 << 16;
const NUMBER: usize = 100_000;
const VALUE_SIZE: u64 = 5;

#[bench]
fn bench_safe_read(b: &mut Bencher) {
    let values = gen_random_vec(NUMBER, 0xdeadbeef);
    let mut array = vec![0; ARRAY_SIZE];

    for (i, v) in values.iter().enumerate() {
        let rnd = v & ((1 << VALUE_SIZE) - 1);
        safe_write(&mut array, i as u64, rnd, VALUE_SIZE);
    }

    b.iter(|| {
        for i in 0..NUMBER {
            black_box(safe_read(&array, i as u64, VALUE_SIZE));
        }
    });
}

#[bench]
fn bench_unsafe_read(b: &mut Bencher) {
    let values = gen_random_vec(NUMBER, 0xdeadbeef);
    let mut array = vec![0; ARRAY_SIZE];

    for (i, v) in values.iter().enumerate() {
        let rnd = v & ((1 << VALUE_SIZE) - 1);
        safe_write(&mut array, i as u64, rnd, VALUE_SIZE);
    }

    b.iter(|| {
        for i in 0..NUMBER {
            black_box(unsafe_read(&array, i as u64, VALUE_SIZE));
        }
    });
}

#[bench]
fn bench_unsafe_write(b: &mut Bencher) {
    let values = gen_random_vec(NUMBER, 0xdeadbeef);
    let mut array = vec![0; ARRAY_SIZE];
    b.iter(|| {
        for (i, v) in values.iter().enumerate() {
            let rnd = v & ((1 << VALUE_SIZE) - 1);
            black_box(unsafe_write(&mut array, i as u64, rnd, VALUE_SIZE));
        }
    });
}

#[bench]
fn bench_safe_write(b: &mut Bencher) {
    let values = gen_random_vec(NUMBER, 0xdeadbeef);
    let mut array = vec![0; ARRAY_SIZE];
    b.iter(|| {
        for (i, v) in values.iter().enumerate() {
            let rnd = v & ((1 << VALUE_SIZE) - 1);
            black_box(safe_write(&mut array, i as u64, rnd, VALUE_SIZE));
        }
    });
}