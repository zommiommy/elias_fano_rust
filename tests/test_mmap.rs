mod utils;
use utils::*;

use elias_fano_rust::prelude::*;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_mmap() {
    // memory map the test graph file
    let mmap = MemoryMappedFileReadOnly::open(
        "./tests/data/cnr-2000/cnr-2000.graph"
    ).unwrap();

    // check that the first 4 bytes match what we expect
    assert_eq!(&mmap[0].to_ne_bytes()[..4], &[0x37, 0x77, 0x85, 0xa7]);

    println!("{:064b}", mmap[0].to_be());

    // create a backend that reads codes from the MSB to the LSb
    let backend =  BitArrayM2L::new(mmap);

    // test single bits reads
    {
        let mut reader = backend.get_codes_reader(0);
        const TRUTH: &[bool] = &[
            false, false, true, true, false, true, true, true, false, true,
            true, true, false, true, true, true, true, false, false, false,
            false, true, false, true, true, false, true, false, false, true,
        ];
        for (i, bit) in TRUTH.iter().enumerate() {
            assert_eq!(i, reader.tell_bits().unwrap());
            assert_eq!(
                *bit, reader.read_bit().unwrap(), 
                "bit read error, index: {}", i
            );
        }
    }

    // test fixed length bits reads
    {
        let mut reader = backend.get_codes_reader(0);
        const TRUTH: &[usize] = &[
            0b001, 0b101, 0b110, 0b111, 0b011, 0b110, 0b000, 0b101, 0b101, 0b001
        ];
        for (i, bits) in TRUTH.iter().enumerate() {
            assert_eq!(3 * i, reader.tell_bits().unwrap());
            assert_eq!(
                *bits, reader.read_fixed_length(3).unwrap(), 
                "fixed length read error, index: {}", i
            );
        }
    }

    let wg = WebGraph::new(RuntimeWebGraphReader::new(
        CodesSettings::default(),
        backend,
    ), vec![0]);

    println!("{:?}", wg.get_degree(0).unwrap());
    println!("{:?}", wg.get_neighbours(0).unwrap());
}