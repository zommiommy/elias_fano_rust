use super::*;

extern crate indexed_bitvec;
use indexed_bitvec::IndexedBits;
use indexed_bitvec::bits::Bits;

pub fn bench_indexed_bitvec_function() {
    let v = test_vector();

    let start_memory = measure_mem();
    let mut bv =Bits::from_bytes(vec![0xFE, 0xFE], 0).unwrap();
    let mut last_v = 0;
    for val  in &v {
        for _ in  last_v..*val {
            bv.push(false);
        }
        bv.push(true);
        last_v = *val;
    }
    let ib = IndexedBits::build_from_bits(bv);

    let end_memory = measure_mem();
    let memory = end_memory - start_memory;

    let mut rank_total_cycles = 0.0;
    let mut seed = 0xdeadbeef;
    for _ in 0..TIME_TRIALS {
        seed = xorshift(seed);
        let index = seed % SIZE;
        let start = rdtsc() as f64;
        
        ib.rank_ones(index);

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

        ib.select_ones(index);

        let end = rdtsc() as f64;
        select_total_cycles += end - start;
    }
    select_total_cycles /= TIME_TRIALS as f64;

    println!("indexed_bitvec,{},{},{}", memory, rank_total_cycles, select_total_cycles);
}