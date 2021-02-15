use super::*;

extern crate bio;
extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;

pub fn bench_bio_function() {
    let v = test_vector();

    let start_memory = measure_mem();

    let mut bv = BitVec::new();
    let mut last_v = 0;
    for val  in &v {
        for _ in  last_v..*val {
            bv.push(false);
        }
        bv.push(true);
        last_v = *val;
    }
    let rs = RankSelect::new(bv, 1);

    let end_memory = measure_mem();
    let memory = end_memory - start_memory;

    let mut rank_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;
        
        rs.rank_1(index);

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

        rs.select_1(index);

        let end = rdtsc() as f64;
        select_total_cycles += end - start;
    }
    select_total_cycles /= TIME_TRIALS as f64;

    println!("bio,{},{},{}", memory, rank_total_cycles, select_total_cycles);
}