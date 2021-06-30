use super::*;

extern crate rsdict;
use rsdict::RsDict;

pub fn bench_rsdict_function() {
    let v = test_vector();

    let start_memory = measure_mem();

    let mut bv = RsDict::new();
    let mut last_v = 0;
    for val  in &v {
        for _ in  last_v..*val {
            bv.push(false);
        }
        bv.push(true);
        last_v = *val;
    }

    let memory = measure_mem() - start_memory;

    let mut rank_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;
        
        bv.rank(index, true);

        let end = rdtsc() as f64;
        rank_total_cycles += end - start;
    }
    rank_total_cycles /= TIME_TRIALS as f64;

    let mut select_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;

        bv.select1(index);

        let end = rdtsc() as f64;
        select_total_cycles += end - start;
    }
    select_total_cycles /= TIME_TRIALS as f64;

    println!("rsdict,{},{},{}", memory, rank_total_cycles, select_total_cycles);
}