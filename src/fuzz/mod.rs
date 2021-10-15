use crate::{
    elias_fano::*,
    sparse_index::*,
    compact_array::*,
    codes::*,
};
use arbitrary::{Arbitrary, Unstructured};
use rayon::iter::*;
use fid::*;

pub fn rank_and_select_harness(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector
    data.sort();

    let ef = EliasFano::<10>::from_vec(&data).unwrap();

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


pub fn si_doubleended_iter_harness(data: &[u8]) {
    let (start, end, fb_nexts, values) = <(u16, u16, Vec<bool>, Vec<u16>)>::arbitrary(&mut Unstructured::new(data)).unwrap();
    // create a sorted vector with no duplicates
    let mut values = values.iter().map(|x| *x as u64).collect::<Vec<_>>();
    values.sort();

    let mut si = SparseIndex::<10>::new();

    let mut iter = si.iter_double_ended();
    let mut truth_iter = values.iter().map(|x| *x);
    
    for fb in &fb_nexts {
        assert_eq!(
            if *fb {
                iter.next()
            } else {
                iter.next_back()
            },
            if *fb {
                truth_iter.next()
            } else {
                truth_iter.next_back()
            },
        )
    }

    let mut iter = si.iter_in_range_double_ended(start as u64..end as u64);
    let mut truth_iter = values.iter().filter(|x| (start as u64..end as u64).contains(x)).map(|x| *x);
    
    for fb in &fb_nexts {
        assert_eq!(
            if *fb {
                iter.next()
            } else {
                iter.next_back()
            },
            if *fb {
                truth_iter.next()
            } else {
                truth_iter.next_back()
            },
        )
    }
}

pub fn iter_harness(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector with no duplicates
    data.sort();

    let ef = EliasFano::<10>::from_vec(&data).unwrap();

    assert_eq!(ef.len() as usize, data.len() as usize, "the length of the vector do not match!");

    for (a, b) in data.iter().zip(ef.iter()) {
        assert_eq!(*a, b, "The values inside elias-fano");
    }

    let seq = ef.iter().collect::<Vec<_>>();
    let par = ef.par_iter().collect::<Vec<_>>();
    assert_eq!(seq, par);
}

pub fn simple_select_harness(data: Vec<bool>) {
    let mut hb = SparseIndex::<10>::new();
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
    
    let InputData {
        start,
        end,
        indices,
    } = data.unwrap();
    

    let mut indices = indices.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector with no duplicates
    indices.sort();

    if indices.len() == 0 {
        return;
    }

    let ef = EliasFano::<10>::from_vec(&indices).unwrap();

    assert_eq!(ef.len() as usize, indices.len() as usize, "the length of the vector do not match!");
    
    let truth = indices.iter().filter(|i| (start..end).contains(&i)).cloned().collect::<Vec<u64>>();

    let ours = ef.iter_in_range(start..end).collect::<Vec<u64>>();

    for (a, b) in truth.iter().zip(ours.iter()) {
        assert_eq!(*a, *b, "The values inside elias-fano");
    }
}

pub fn ef_builder_harness(data: &[u8]) {
    let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
    let mut data = data.iter().map(|x| *x as u64).collect::<Vec<u64>>();
    // create a sorted vector
    data.sort();

    let ef = EliasFano::<10>::from_vec(&data).unwrap();

    assert_eq!( ef.len() as usize, data.len() as usize, "the sequential elias fano length do not match the vector!");

    let cefb = ConcurrentEliasFanoBuilder::<10>::new(data.len() as u64, *data.last().unwrap_or(&0)).unwrap();

    data.par_iter().enumerate().for_each(|(i, x)| cefb.set(i as u64, *x));

    let cef = cefb.build().unwrap();

    assert_eq!(cef.len() as usize, data.len() as usize, "the concurrent elias fano length do not match the vector!");
    
    for ((t, s), c) in data.iter().zip(ef.iter()).zip(cef.iter()) {
        assert_eq!(*t, s, "The sequetial iter do not match the truth data.");
        assert_eq!(*t, c, "The concurrent iter do not match the truth data.");
    }
}

pub fn codes_harness(data: &[u8]) {
    let data = <Vec<(u8, u64)>>::arbitrary(&mut Unstructured::new(data));
    if data.is_err() {
        return;
    }
    let data = data.unwrap();

    let mut bs = BitStream::new();

    for (t, v) in data.iter() {
        match *t % 9 {
            0 => bs.write_unary(*v),
            1 => bs.write_zeta::<1>(*v),
            2 => bs.write_zeta::<2>(*v),
            3 => bs.write_zeta::<3>(*v),
            4 => bs.write_zeta::<4>(*v),
            5 => bs.write_zeta::<5>(*v),
            6 => bs.write_zeta::<6>(*v),
            7 => bs.write_zeta::<7>(*v),
            8 => bs.write_zeta::<8>(*v),
            9 => bs.write_gamma(*v),
            _ => unreachable!(),
        };
    }

    bs.seek(0);

    for (t, v) in data.iter() {
        assert_eq!(
            *v, 
            match *t % 9 {
                0 => bs.read_unary(),
                1 => bs.read_zeta::<1>(),
                2 => bs.read_zeta::<2>(),
                3 => bs.read_zeta::<3>(),
                4 => bs.read_zeta::<4>(),
                5 => bs.read_zeta::<5>(),
                6 => bs.read_zeta::<6>(),
                7 => bs.read_zeta::<7>(),
                8 => bs.read_zeta::<8>(),
                9 => bs.read_gamma(),
                _ => unreachable!(),
            }
        );
    }
}