#![feature(test)]
#![allow(clippy::unreadable_literal)]

extern crate rand;

use rand::{Rng, SeedableRng};
use rand::{RngCore};
use rand::rngs::SmallRng;

extern crate test;
use test::{Bencher, black_box};

const TRIALS: u64 = 1_000;
//const SIZE: u64 = 1_000_000;
const SIZE: u64 = 32_000_000;
const MAX : u64 = 450_000 * 450_000;
//const MAX: u64 = 2 * SIZE;

const SEED: [u8; 16] = [
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe,
    0xde, 0xad, 0xbe, 0xef,
    0xc0, 0xfe, 0xbe, 0xbe   
];

mod ef {
    use super::*;
        
    #[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
        println!("{:#4?}", ef.memory_stats());
        println!("{:?}", ef.size());
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(ef.rank(rng.gen_range(0, SIZE)));
            }
        })
    }

    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ef = elias_fano_rust::EliasFano::from_vec(&v).unwrap();
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(ef.select(rng.gen_range(0, SIZE)).unwrap());
            }
        })
    }
}


mod simple_select {
    use super::*;
        
    #[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ss = elias_fano_rust::SimpleSelect::from_vec(v);
        println!("{:?}", ss.size());
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(ss.rank1(rng.gen_range(0, SIZE)));
            }
        })
    }

    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let ss = elias_fano_rust::SimpleSelect::from_vec(v);
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(ss.select1(rng.gen_range(0, SIZE)));
            }
        })
    }
}

pub(crate) fn test_vector() -> (Vec<u64>, SmallRng) {
    let mut rng: SmallRng = SmallRng::from_seed(SEED);
    let mut v = Vec::new();
    for _ in 0..SIZE {
        v.push(rng.next_u64() % MAX);
    }
    v.sort();
    (v, rng)
}

mod fid {
    use super::*;
    extern crate fid;
    use fid::{BitVector, FID};

    #[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = BitVector::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(bv.rank1(rng.gen_range(0, SIZE)));
            }
        })
    }
    
    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = BitVector::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(bv.select1(rng.gen_range(0, SIZE)));
            }
        })
    }
}

mod rsdict {
    use super::*;
    extern crate rsdict;
    use rsdict::RsDict;

    #[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = RsDict::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(bv.rank(rng.gen_range(0, SIZE), true));
            }
        })
    }
    
    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = RsDict::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(bv.select1(rng.gen_range(0, SIZE)));
            }
        })
    }
}

mod succint {
    use super::*;
    extern crate succinct;
    use succinct::BitVector;
    use succinct::bit_vec::BitVecPush;
    use succinct::rank::{
        Rank9,
        JacobsonRank
    }; 
    use succinct::rank::BitRankSupport;
    use succinct::BinSearchSelect;
    use succinct::select::Select1Support;
    use succinct::broadword::Broadword;

    //#[bench]
    fn rank9_rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv: BitVector<u64> = BitVector::new();
        let mut last_v = 0;
            for val  in v {
                for _ in  last_v..val {
                    bv.push_bit(false);
                }
                bv.push_bit(true);
                last_v = val;
            }
        let r = Rank9::new(bv);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(r.rank1(rng.gen_range(0, SIZE)));
            }
        })
    }   

    //#[bench]
    fn rank9_select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv: BitVector<u64> = BitVector::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push_bit(false);
            }
            bv.push_bit(true);
            last_v = val;
        }
        let r = Rank9::new(bv);
        let s = BinSearchSelect::new(r);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(s.select1(rng.gen_range(0, SIZE)));
            }
        })
    }   

    //#[bench]
    fn jacobson_rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv: BitVector<u64> = BitVector::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push_bit(false);
            }
            bv.push_bit(true);
            last_v = val;
        }
        let r = JacobsonRank::new(bv);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(r.rank1(rng.gen_range(0, SIZE)));
            }
        })
    }   
    
    //#[bench]
    fn jacobson_select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv: BitVector<u64> = BitVector::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push_bit(false);
            }
            bv.push_bit(true);
            last_v = val;
        }
        let r = JacobsonRank::new(bv);
        let s = BinSearchSelect::new(r);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(s.select1(rng.gen_range(0, SIZE)));
            }
        })
    }
}

mod indexed_bitvec {
    use super::*;
    extern crate indexed_bitvec;
    use indexed_bitvec::IndexedBits;
    use indexed_bitvec::bits::Bits;

    //#[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv =Bits::from_bytes(vec![0xFE, 0xFE], 0).unwrap();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        let ib = IndexedBits::build_from_bits(bv);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(ib.rank_ones(rng.gen_range(0, SIZE)));
            }
        })
    }   

    //#[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv =Bits::from_bytes(vec![0xFE, 0xFE], 0).unwrap();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        let ib = IndexedBits::build_from_bits(bv);
        b.iter(|| {
            for _ in 0..super::TRIALS {
                black_box(ib.select_ones(rng.gen_range(0, SIZE)));
            }
        })
    }   
}


mod z_bio {
    use super::*;
    extern crate bio;
    extern crate bv;
    use bio::data_structures::rank_select::RankSelect;
    use bv::BitVec;
    

    //#[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = BitVec::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        let rs = RankSelect::new(bv, 1);
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(rs.rank_1(rng.gen_range(0, SIZE)));
            }
        })
    }

    //#[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let mut bv = BitVec::new();
        let mut last_v = 0;
        for val  in v {
            for _ in  last_v..val {
                bv.push(false);
            }
            bv.push(true);
            last_v = val;
        }
        let rs = RankSelect::new(bv, 1);
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(rs.select_1(rng.gen_range(0, SIZE)));
            }
        })
    }
}


mod vec {
    use super::*;
    
    #[bench]
    fn rank(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(v.binary_search(&rng.gen_range(0, SIZE)));
            }
        })
    }

    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(v[rng.gen_range(0, SIZE) as usize]);
            }
        })
    }
}

mod hashmap {
    use super::*;
    use std::collections::HashMap;
    
    #[bench]
    fn select(b: &mut Bencher) {
        let (v, mut rng) = test_vector();
        let m : HashMap<usize, u64> = v.iter().enumerate().map(|(i, v)| (i, *v)).collect();
        b.iter(|| {
            for _ in 0..TRIALS {
                black_box(m.get(&(rng.gen_range(0, SIZE) as usize)));
            }
        })
    }
}