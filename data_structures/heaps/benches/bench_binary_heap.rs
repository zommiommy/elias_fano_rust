#![feature(test, portable_simd)]
extern crate test;
use test::{black_box, Bencher};
use std::simd::u64x8;

// our implementation
extern crate heaps;
use heaps::BinaryHeap as MyBinaryHeap;
use heaps::KAryHeap;
use heaps::VectorizedBinaryHeap;

// the standard reference implementation
use std::collections::BinaryHeap;
use std::cmp::Reverse;


// random number generators
mod utils;
use utils::*;

// Constants for benching
const SIZE: usize = 0x10_000;
const MAX: u64 = u64::MAX;


mod my_binary_heap {
    use super::*;
    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut my_heap = MyBinaryHeap::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = MyBinaryHeap::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
}

mod vectorized_binary_heap {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut my_heap = VectorizedBinaryHeap::with_capacity(SIZE);
        
            for val in data.chunks(8) {
                let reg = u64x8::from_slice(val);
                let _ = black_box(my_heap.push(reg));
            }
        });
    }

    
}


mod kary_heap_2 {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<2>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<2>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
    
}

mod kary_heap_4 {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<4>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<4>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
    
}

mod kary_heap_8 {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<8>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<8>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
    
}

mod kary_heap_16 {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<16>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KAryHeap::<16>::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(my_heap.push(*val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
    
}

mod std_heap {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut true_heap = BinaryHeap::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(true_heap.push(Reverse(val)));
            }
        });
    }
    
    
    
    
    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut true_heap = BinaryHeap::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(true_heap.push(Reverse(val)));
            }
    
            while !true_heap.is_empty() {
                let _ = black_box(true_heap.pop());
            }
        });
    }
}

/*
mod vec_heap {
    use super::*;

    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut vec = Vec::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(vec.push(*val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut vec = Vec::with_capacity(SIZE);
        
            for val in &data {
                let _ = black_box(vec.push(*val));
            }

            while !vec.is_empty() {
                let (idx, min) = vec.iter().enumerate().min_by_key(|(i, x)| *x).unwrap();
                let number_of_elements = vec.len();
                vec.swap(idx, number_of_elements - 1);
                let _ = black_box(vec.pop());
            }
        });
    }
}
*/