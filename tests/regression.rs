use elias_fano_rust::{EliasFano, fuzz_harness::*};
mod utils;
use rand::AsByteSliceMut;
use utils::*;


#[test]
fn tet_regression() {    
    let data = vec![0x46, 0xca, 0x14, 0x2d];
    rank_and_select_harness(&data);

    let ef = EliasFano::from_vec(&vec![1, 4, 6, 9]).unwrap();
    let _  = ef.iter_in_range(4..8).collect::<Vec<_>>();

    let data = vec![0x81, 0xaf, 0x02, 0x6b, 0x00, 0x1c, 0xf1, 0x02, 0x34, 0x31, 0x1f, 0x45, 0x2f, 0x2f, 0x27, 0x5e, 0xe7, 0xc0, 0x88, 0x00, 0x00];
    iter_in_range_harness(&data);


    let data = vec![                      
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        true,              
        false,             
        true,              
        true,              
        true,              
        true,              
    ];

    simple_select_harness(data.to_vec());
}