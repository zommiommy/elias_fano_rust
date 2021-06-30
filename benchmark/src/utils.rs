use super::*;

use core::arch::x86_64::__rdtscp;
use procfs::Meminfo;

/// return the number of CPU cicles
pub fn rdtsc() -> u64 {
    let mut _aux = 0;
    unsafe{
        __rdtscp(&mut _aux)
    }
}

/// return the currently used memory in bytes
pub fn read_mem() -> f64 {
    let mem = Meminfo::new().unwrap();
    (mem.mem_total - mem.mem_free - mem.buffers - mem.cached - mem.slab) as f64 / (1024 * 1024) as f64
}

pub fn xorshift(mut x: u64) -> u64 {
	x ^= x << 13;
	x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn measure_mem() -> f64 {
    let mut total = 0.0;
    for _ in 0..MEM_TRIALS {
        total += read_mem();
        thread::sleep(time::Duration::from_millis(200));  
    }
    total / MEM_TRIALS as f64
}

const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe,
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe   
];

pub fn test_vector() -> Vec<u64> {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..SIZE {
        v.push(rng.next_u64() % MAX);
    }
    v.sort();
    v
}