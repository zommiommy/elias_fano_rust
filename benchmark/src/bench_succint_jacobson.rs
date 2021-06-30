use super::*;

extern crate succinct;
use succinct::BitVector;
use succinct::bit_vec::BitVecPush;
use succinct::rank::{
    JacobsonRank
}; 
use succinct::rank::BitRankSupport;
use succinct::BinSearchSelect;
use succinct::select::Select1Support;

pub fn bench_succint_jacobson_function() {
    let v = test_vector();

    let start_memory = measure_mem();

    let mut bv: BitVector<u64> = BitVector::new();
    let mut last_v = 0;
    for val in &v {
        for _ in  last_v..*val {
            bv.push_bit(false);
        }
        bv.push_bit(true);
        last_v = *val;
    }
    let r = JacobsonRank::new(bv);
    let s = BinSearchSelect::new(r);

    let end_memory = measure_mem();
    let memory = end_memory - start_memory;

    let mut rank_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;
        
        s.rank1(index);

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

        s.select1(index);

        let end = rdtsc() as f64;
        select_total_cycles += end - start;
    }
    select_total_cycles /= TIME_TRIALS as f64;

    println!("succint_jacobson,{},{},{}", memory, rank_total_cycles, select_total_cycles);
}