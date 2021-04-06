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
    
    for i in 0..data.len() {
        let truth = data[i];
        let ours = ef.unchecked_select(i as u64);
        assert_eq!(
            truth, 
            ours,
            concat!(
                "The  selects are different!\n",
                "The truth is {} while we returned {} as the select of index {}"
            ),
            truth, 
            ours,
            i,
        );
    }

    for x in &data {
        let mut truth = data.binary_search(x).unwrap() as u64;
        while truth > 0 && data[truth as usize - 1] == *x {
            truth -= 1;
        }
        
        let ours = ef.unchecked_rank(*x);
        assert_eq!(
            truth, 
            ours,
            concat!(
                "The ranks are different!\n",
                "The truth is {} while we returned {} as the rank of value {}"
            ),
            truth, 
            ours,
            x,
        );
    }

});
