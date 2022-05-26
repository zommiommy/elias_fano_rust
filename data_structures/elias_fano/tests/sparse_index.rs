use elias_fano_rust::sparse_index::SparseIndex;
use elias_fano_rust::traits::*;
use fid::*;

use rand::Rng;

const ITERS: usize = 1_000_000;

#[test]
/// Test that we encode and decode low bits properly.
fn test_simple_select_against_fid() {
    let mut rng = rand::thread_rng();

    // initialize the two data structures
    let mut hb = SparseIndex::<10>::new();
    let mut rs = BitVector::new();

    // generate a dense bitvector
    for i in 0..ITERS {
        let bit = rng.gen_bool(0.5);
        hb.push(bit);
        rs.push(bit);
        // assert that the bit were correctly assigned
        assert_eq!(bit, rs.get(i as u64));
        assert_eq!(bit, hb.get(i));
    }

    // check all the basic operations
    for i in 0..ITERS {
        assert_eq!(hb.get(i), rs.get(i as u64));
    }

    for i in 0..rs.rank1(rs.len()) {
        assert_eq!(
            hb.select1(i as usize) as u64,
            rs.select1(i),
            "error seleting the {}-th one",
            i
        );
    }

    for i in 0..rs.rank0(rs.len()) {
        assert_eq!(
            hb.select0(i as usize) as u64,
            rs.select0(i),
            "error seleting the {}-th zero",
            i
        );
    }

    for i in 0..rs.len() {
        assert_eq!(
            hb.rank1(i as usize) as u64,
            rs.rank1(i),
            "error ranking ones up to {}",
            i
        );
    }

    for i in 0..rs.len() {
        assert_eq!(
            hb.rank0(i as usize) as u64,
            rs.rank0(i),
            "error ranking zeros up to {}",
            i
        );
    }

    // check that ranks and counts of zeros and ones are coherent
    assert_eq!(hb.rank1(hb.len()), hb.count_ones());
    assert_eq!(hb.rank0(hb.len()), hb.count_zeros());

    println!(
        "Simple select uses: {} Mib",
        hb.total_size() as f64 / (1024.0 * 1024.0)
    );
    println!("{:#4?}", hb.total_size());

    // test random access on the iterator
    let mut truth_iter = (0..rs.rank1(rs.len())).map(|i| rs.select1(i));
    let mut test_iter = hb.iter_double_ended();
    // check that the compute size is correct
    assert_eq!(test_iter.size_hint().0, rs.rank1(rs.len()) as usize);
    println!("{:#4?}", test_iter);
    loop {
        let bit = rng.gen_bool(0.5);
        if bit {
            let truth = truth_iter.next();
            let test = test_iter.next().map(|x| x as u64);
            assert_eq!(truth, test, "fw: {:#4?}", test_iter);
            if truth.is_none() && test.is_none() {
                break;
            }
        } else {
            let truth = truth_iter.next_back();
            let test = test_iter.next_back().map(|x| x as u64);
            assert_eq!(truth, test, "bw: {:#4?}", test_iter);
            if truth.is_none() && test.is_none() {
                break;
            }
        }
    }

    // test random access on the iterator in a range
    let (start, end) = (rng.gen_range(0, rs.len()), rng.gen_range(0, rs.len()));
    let (start, end) = (start.min(end), start.max(end));
    println!("{}..{}", start, end);
    let mut truth_iter = (0..rs.rank1(rs.len()))
        .map(|i| rs.select1(i))
        .filter(|i| (start..end).contains(i));
    let mut test_iter = hb.iter_in_range_double_ended(start as usize..end as usize);
    // check that the compute size is correct
    assert_eq!(
        test_iter.size_hint().0,
        (0..rs.rank1(rs.len()))
            .map(|i| rs.select1(i))
            .filter(|i| (start..end).contains(i))
            .count()
    );

    println!("{:#4?}", test_iter);
    loop {
        let bit = rng.gen_bool(0.5);
        if bit {
            let truth = truth_iter.next();
            let test = test_iter.next().map(|x| x as u64);
            assert_eq!(truth, test, "fw: {:#4?}", test_iter);
            if truth.is_none() && test.is_none() {
                break;
            }
        } else {
            let truth = truth_iter.next_back();
            let test = test_iter.next_back().map(|x| x as u64);
            assert_eq!(truth, test, "bw: {:#4?}", test_iter);
            if truth.is_none() && test.is_none() {
                break;
            }
        }
    }
}
