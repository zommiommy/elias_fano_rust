use super::*;

#[cfg(not(feature="fuzz"))]
pub mod fuzz_harness{}

#[cfg(feature="fuzz")]
pub mod fuzz_harness{
    use arbitrary::{Arbitrary, Unstructured};
    use super::*;

    pub fn rank_and_select_harness(data: &[u8]) {
        let data = <Vec<u16>>::arbitrary(&mut Unstructured::new(data)).unwrap();
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
    
        let ours = ef.iter_in_range(start..end).collect::<Vec<u64>>();
    
        for (a, b) in truth.iter().zip(ours.iter()) {
            assert_eq!(*a, *b, "The values inside elias-fano");
        }
    }
}