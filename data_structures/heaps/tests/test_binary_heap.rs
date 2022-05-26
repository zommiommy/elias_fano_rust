
// our implementation
use heaps::BinaryHeap as MyBinaryHeap;

// the standard reference implementation
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// random number generators
mod utils;
use utils::*;

#[test]
/// Test that our implementation is correct
fn test_binary_heap() {
    let data = build_random_vector(1_000, 10_000);

    let mut true_heap = BinaryHeap::new();
    let mut my_heap = MyBinaryHeap::new();

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
