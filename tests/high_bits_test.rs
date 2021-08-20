use elias_fano_rust::*;
use rsdict::*;

use rand::Rng;

const ITERS: usize = 1_000_000;

#[test]
/// Test that we encode and decode low bits properly.
fn test_high_bits_against_rsdict() {
    let mut rng = rand::thread_rng();
    
    let mut hb = SimpleSelect::new();
    let mut rs = RsDict::new();

    for i in 0..ITERS {
        let bit = rng.gen_bool(0.5);
        hb.push(bit);
        rs.push(bit);
        assert_eq!(bit, rs.get_bit(i as u64));
        assert_eq!(bit, hb.get(i as u64));
    }

    for i in 0..ITERS {
        assert_eq!(hb.get(i as u64), rs.get_bit(i as u64));
    }

    for i in 0..rs.count_ones() as u64 {
        assert_eq!(hb.select1(i), rs.select1(i).unwrap(), "error seleting the {}-th one", i);
    }

    for i in 0..rs.count_zeros() as u64 {
        assert_eq!(hb.select0(i), rs.select0(i).unwrap(), "error seleting the {}-th zero", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank1(i), rs.rank(i, true), "error ranking ones up to {}", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank0(i), rs.rank(i, false), "error ranking zeros up to {}", i);
    }

    assert_eq!(hb.rank1(hb.len()), hb.count_ones());
    assert_eq!(hb.rank0(hb.len()), hb.count_zeros());

    println!("Simple select uses: {} Mib", hb.size().total() as f64 / (1024.0*1024.0));
    println!("{:#4?}", hb.size());
}
