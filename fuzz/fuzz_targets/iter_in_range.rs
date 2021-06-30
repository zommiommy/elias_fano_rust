#![no_main]
use libfuzzer_sys::fuzz_target;
use fid::{FID, BitVector};
use elias_fano_rust::EliasFano;
use arbitrary::{Arbitrary, Unstructured};

#[derive(Arbitrary, Debug)]
struct InputData {
    start: u64,
    end: u64,
    indices: Vec<u16>,
}

fuzz_target!(|data: &[u8]| {

    let data = InputData::arbitrary(&mut Unstructured::new(data));
    if data.is_err() {
        return;
    }


    let InputData {
        start,
        end,
        mut indices,
    } = data.unwrap();
    

    let mut indices = indices.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector with no duplicates
    indices.sort();

    if indices.len() == 0 {
        return;
    }

    let ef = EliasFano::from_vec(&indices).unwrap();

    assert_eq!(ef.len() as usize, indices.len() as usize, "the length of the vector do not match!");
    
    let truth = indices.iter().filter(|i| (start..end).contains(&i)).cloned().collect::<Vec<u64>>();

    let ours = ef.iter_in_range(start..end).collect::<Vec<u64>>();

    for (a, b) in truth.iter().zip(ours.iter()) {
        assert_eq!(*a, *b, "The values inside elias-fano");
    }

});
