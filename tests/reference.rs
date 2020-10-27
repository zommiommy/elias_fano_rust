use elias_fano_rust::EliasFano;
mod utils;
use utils::*;

extern crate rsdict;
use rsdict::RsDict;
extern crate fid;
use fid::{FID, BitVector};

#[test]
fn test_reference() {    
    let mut r = RsDict::new();
    for _ in 0..65 {
        r.push(true);
    }
    
    // these fail
    r.select(3, true);
}