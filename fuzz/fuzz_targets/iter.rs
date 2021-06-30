#![no_main]
use libfuzzer_sys::fuzz_target;
use fid::{FID, BitVector};
use elias_fano_rust::EliasFano;

fuzz_target!(|data: Vec<u16>| {
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector with no duplicates
    data.sort();

    let ef = EliasFano::from_vec(&data).unwrap();

    assert_eq!(ef.len() as usize, data.len() as usize, "the length of the vector do not match!");
    
    for (a, b) in data.iter().zip(ef.iter()) {
        assert_eq!(*a, b, "The values inside elias-fano");
    }

});
