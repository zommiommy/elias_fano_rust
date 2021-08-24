use elias_fano_rust::*;
use fid::*;

use rand::Rng;

const ITERS: usize = 1_000_000;

#[test]
/// Test that we encode and decode low bits properly.
fn test_simple_select_against_fid() {
    let mut rng = rand::thread_rng();

    // initialize the two data structures
    let mut hb = SimpleSelect::new();
    let mut rs = BitVector::new();

    // generate a dense bitvector
    for i in 0..ITERS {
        let bit = rng.gen_bool(0.5);
        hb.push(bit);
        rs.push(bit);
        // assert that the bit were correctly assigned
        assert_eq!(bit, rs.get(i as u64));
        assert_eq!(bit, hb.get(i as u64));
    }

    // check all the basic operations
    for i in 0..ITERS {
        assert_eq!(hb.get(i as u64), rs.get(i as u64));
    }

    for i in 0..rs.rank1(rs.len()) {
        assert_eq!(hb.select1(i), rs.select1(i), "error seleting the {}-th one", i);
    }

    for i in 0..rs.rank0(rs.len()) {
        assert_eq!(hb.select0(i), rs.select0(i), "error seleting the {}-th zero", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank1(i), rs.rank1(i), "error ranking ones up to {}", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank0(i), rs.rank0(i), "error ranking zeros up to {}", i);
    }

    // check that ranks and counts of zeros and ones are coherent
    assert_eq!(hb.rank1(hb.len()), hb.count_ones());
    assert_eq!(hb.rank0(hb.len()), hb.count_zeros());

    println!("Simple select uses: {} Mib", hb.size().total() as f64 / (1024.0*1024.0));
    println!("{:#4?}", hb.size());

    // test normal iteration
    let truth_iter = (0..rs.rank1(rs.len())).map(|i| rs.select1(i));
    let test_iter = hb.iter_double_ended();
    for (truth, test) in truth_iter.zip(test_iter) {
        assert_eq!(truth, test);
    }

    // test reverse iteration
    let truth_iter = (0..rs.rank1(rs.len())).map(|i| rs.select1(i)).rev();
    let test_iter = hb.iter_double_ended().rev();
    for (truth, test) in truth_iter.zip(test_iter) {
        assert_eq!(truth, test);
    }

    // test random access on the iterator
    let mut truth_iter = (0..rs.rank1(rs.len())).map(|i| rs.select1(i));
    let mut test_iter = hb.iter_double_ended();
    for i in 0..hb.count_ones() {
        let bit = rng.gen_bool(0.5);
        if bit {
            assert_eq!(truth_iter.next(), test_iter.next(), "{:#4?}", test_iter);
        } else {
            assert_eq!(truth_iter.next_back(), test_iter.next_back(), "{:#4?}", test_iter);
        }
    }
}
