use super::*;

use std::collections::HashMap;

pub fn bench_hashmap_function() {
    let v = test_vector();

    let start_memory = measure_mem();

    let m : HashMap<usize, u64> = v.iter().enumerate().map(|(i, v)| (i, *v)).collect();

    let end_memory = measure_mem();
    let memory = end_memory - start_memory;

    let mut select_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;

        m.get(&(index as usize)).unwrap();

        let end = rdtsc() as f64;
        select_total_cycles += end - start;
    }
    select_total_cycles /= TIME_TRIALS as f64;

    println!("hashmap,{},{},{}", memory, -1, select_total_cycles);
}