extern crate rand;
use std::{thread, time};
use rand::{SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;

mod utils;
use utils::*;
mod constants;
use constants::*;
mod bench_fid;
use bench_fid::bench_fid_function;
mod bench_rsdict;
use bench_rsdict::bench_rsdict_function;
mod bench_ef;
use bench_ef::bench_ef_function;
mod bench_hashmap;
use bench_hashmap::bench_hashmap_function;
mod bench_vec;
use bench_vec::bench_vec_function;
mod bench_bio;
use bench_bio::bench_bio_function;
mod bench_indexed_bitvec;
use bench_indexed_bitvec::bench_indexed_bitvec_function;
mod bench_succint_rank9;
use bench_succint_rank9::bench_succint_rank9_function;
mod bench_succint_jacobson;
use bench_succint_jacobson::bench_succint_jacobson_function;

const SLEEP_MS: u64 = 100;

fn main() {
    let funcs = [
        bench_rsdict_function,
        bench_vec_function,
        bench_ef_function,
        bench_fid_function,
        bench_bio_function,
        bench_indexed_bitvec_function,
        bench_succint_rank9_function,
        bench_succint_jacobson_function,
        bench_hashmap_function,
    ];

    let start_memory = measure_mem();
    eprintln!("Starting memory: {}", start_memory);

    println!("library,memory,rank,select");

    for func in funcs.iter() {
        func();
        thread::sleep(time::Duration::from_millis(SLEEP_MS)); 
        let mut curr_mem = measure_mem();
        while (curr_mem - start_memory).abs() > 100.0 {
            thread::sleep(time::Duration::from_millis(SLEEP_MS)); 
            curr_mem = measure_mem();
        }
    }
}
