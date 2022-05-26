#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

// our implementation
extern crate heaps;
use heaps::BinaryHeap as MyBinaryHeap;
use heaps::KAryHeap;
use heaps::{Element, GenericPriorityQueue};
use heaps::PriorityQueue;

// the standard reference implementation
use std::collections::BinaryHeap;
use std::cmp::Reverse;


// random number generators
mod utils;
use utils::*;

// Constants for benching
const SIZE: usize = 10_000;
const MAX: u64 = u64::MAX;

mod generic_priority_queue {
    use super::*;
    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut my_heap = GenericPriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(Element{key:i as u64, value:*val}));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = GenericPriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(Element{key:i as u64, value:*val}));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
}

mod priority_queue {
    use super::*;
    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut my_heap = PriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(i as u64, *val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = PriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(i as u64, *val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
}

mod keyed_priority_heap {
    use keyed_priority_queue::{KeyedPriorityQueue, Entry};
    use super::*;
    #[bench]
    fn push(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);

        b.iter(|| {
            let mut my_heap = KeyedPriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(i, *val));
            }
        });
    }

    #[bench]
    fn push_and_pop(b: &mut Bencher) {
        let data = build_random_vector(SIZE, MAX);
    
        b.iter(|| {
            let mut my_heap = KeyedPriorityQueue::with_capacity(SIZE);
        
            for (i, val) in data.iter().enumerate() {
                let _ = black_box(my_heap.push(i, *val));
            }
    
            while !my_heap.is_empty() {
                let _ = black_box(my_heap.pop());
            }
        });
    }
}