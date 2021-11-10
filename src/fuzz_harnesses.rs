
use super::*;
use arbitrary::{Arbitrary, Unstructured};
use rayon::iter::*;
use fid::*;

pub fn rank_and_select_harness(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector
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
}


pub fn iter_harness(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector with no duplicates
    data.sort();

    let ef = EliasFano::from_vec(&data).unwrap();

    assert_eq!(ef.len() as usize, data.len() as usize, "the length of the vector do not match!");

    for (a, b) in data.iter().zip(ef.iter()) {
        assert_eq!(*a, b, "The values inside elias-fano");
    }

    let seq = ef.iter().collect::<Vec<_>>();
    let par = ef.par_iter().collect::<Vec<_>>();
    assert_eq!(seq, par);
}

pub fn simple_select_harness(data: Vec<bool>) {
    let mut hb = SimpleSelect::new();
    let mut rs = BitVector::new();

    for bit in data {
        hb.push(bit);
        rs.push(bit);
    }

    for i in 0..rs.rank1(rs.len()) as u64 {
        assert_eq!(hb.select1(i), rs.select1(i), "error seleting the {}-th one", i);
    }

    for i in 0..rs.rank0(rs.len()) as u64 {
        assert_eq!(hb.select0(i), rs.select0(i), "error seleting the {}-th zero", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank1(i), rs.rank1(i), "error ranking ones up to {}", i);
    }

    for i in 0..rs.len() as u64 {
        assert_eq!(hb.rank0(i), rs.rank0(i), "error ranking zeros up to {}", i);
    }
}


#[derive(Arbitrary, Debug)]
struct InputData {
    start: u64,
    end: u64,
    indices: Vec<u16>,
}

pub fn iter_in_range_harness(data: &[u8]) {
    let data = InputData::arbitrary(&mut Unstructured::new(data));
    if data.is_err() {
        return;
    }

    dbg!(&data);
    
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

    let ours = ef.iter_in_range(start..end, None).collect::<Vec<u64>>();

    for (a, b) in truth.iter().zip(ours.iter()) {
        assert_eq!(*a, *b, "The values inside elias-fano");
    }
}

pub fn builders(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector
    data.sort();

    let ef = EliasFano::from_vec(&data).unwrap();

    assert_eq!( ef.len() as usize, data.len() as usize, "the sequential elias fano length do not match the vector!");

    let cefb = ConcurrentEliasFanoBuilder::new(data.len() as u64, *data.last().unwrap_or(&0)).unwrap();

    data.par_iter().enumerate().for_each(|(i, x)| cefb.set(i as u64, *x));

    let cef = cefb.build().unwrap();

    assert_eq!(cef.len() as usize, data.len() as usize, "the concurrent elias fano length do not match the vector!");
    
    for ((t, s), c) in data.iter().zip(ef.iter()).zip(cef.iter()) {
        assert_eq!(*t, s, "The sequetial iter do not match the truth data.");
        assert_eq!(*t, c, "The concurrent iter do not match the truth data.");
    }
}