
// our implementation
use heaps::KAryHeap;

// the standard reference implementation
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// random number generators
mod utils;
use utils::*;

#[test]
/// Test that our implementation is correct
fn test_kary_heap_2() {
    let data = build_random_vector(1_000, 10_000);

    let mut true_heap = BinaryHeap::new();
    let mut my_heap = KAryHeap::<2>::new();

    for val in &data {
        true_heap.push(Reverse(val));
        my_heap.push(*val);
    }

    while !true_heap.is_empty() {
        assert_eq!(
            true_heap.pop().map(|x| *x.0), 
            my_heap.pop()
        );
    }
}


#[test]
/// Test that our implementation is correct
fn test_kary_heap_4() {
    let data = build_random_vector(1_000, 10_000);

    let mut true_heap = BinaryHeap::new();
    let mut my_heap = KAryHeap::<4>::new();

    for val in &data {
        true_heap.push(Reverse(val));
        my_heap.push(*val);
    }

    while !true_heap.is_empty() {
        assert_eq!(
            true_heap.pop().map(|x| *x.0), 
            my_heap.pop()
        );
    }
}

#[test]
/// Test that our implementation is correct
fn test_kary_heap_8() {
    let data = build_random_vector(1_000, 10_000);

    let mut true_heap = BinaryHeap::new();
    let mut my_heap = KAryHeap::<8>::new();

    for val in &data {
        true_heap.push(Reverse(val));
        my_heap.push(*val);
    }

    while !true_heap.is_empty() {
        assert_eq!(
            true_heap.pop().map(|x| *x.0), 
            my_heap.pop()
        );
    }
}

#[test]
/// Test that our implementation is correct
fn test_kary_heap_16() {
    let data = build_random_vector(1_000, 10_000);

    let mut true_heap = BinaryHeap::new();
    let mut my_heap = KAryHeap::<16>::new();

    for val in &data {
        true_heap.push(Reverse(val));
        my_heap.push(*val);
    }

    while !true_heap.is_empty() {
        assert_eq!(
            true_heap.pop().map(|x| *x.0), 
            my_heap.pop()
        );
    }
}

