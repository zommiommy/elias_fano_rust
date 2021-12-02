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

    println!("{:064b} 0x{:016x}", mmap[0].to_be(), mmap[0].to_be());
    println!("{:064b} 0x{:016x}", mmap[1].to_be(), mmap[1].to_be());

    // create a backend that reads codes from the MSB to the LSb
    let backend =  BitArrayM2L::new(mmap);

    // test single bits reads
    {
        let mut reader = (&backend).get_codes_reader(0);
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
        let mut reader = (&backend).get_codes_reader(0);
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

    // test unary bits reads
    {
        let mut reader = (&backend).get_codes_reader(0);
        const TRUTH: &[usize] = &[
            2, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 4
        ];
        let mut offset = 0;
        for bits in TRUTH {
            assert_eq!(
                *bits, reader.read_unary().unwrap(), 
                "unary read error, value: {}", bits
            );
            offset += bits + 1;
            assert_eq!(offset, reader.tell_bits().unwrap());
        }
    }
}

use std::time::Instant;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_cnr_2000() {
    let start = Instant::now();
    // memory map the test graph file
    let mmap = MemoryMappedFileReadOnly::open(
        "./tests/data/cnr-2000/cnr-2000.graph"
    ).unwrap();

    // create a backend that reads codes from the MSB to the LSb
    let backend =  BitArrayM2L::new(mmap);

    let mut wg = WebGraph::new(RuntimeWebGraphReader::new(
        CodesSettings::default(),
        &backend,
    ), vec![0]);

    let truth = std::fs::read_to_string(
        "./tests/data/cnr-2000/cnr-2000_ascii.graph-txt"
    ).unwrap();

    // load the decoded graph for comparison
    let graph = truth.lines().skip(1).map(|x| {
        let mut n = x.split(" ")
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(
                        x.parse::<usize>()
                            .expect(&format!("cannot parse: {}", x))
                    )
                }})
            .collect::<Vec<usize>>();
        n.sort();
        n
    }).collect::<Vec<Vec<usize>>>();

    let mut edges = 0;
    for node_id in 0..325557 {
        let (offset, neighbours) = 
            wg.get_neighbours(node_id).unwrap();

        assert_eq!(graph[node_id], neighbours);
        wg.push_offset(offset);
        
        edges += neighbours.len();
        if (node_id & 0xffff) == 0 {
            println!(
                "[{:13} edges] [{:.3} Medges/sec]", 
                edges, 
                (edges as f64 / 1_000_000.0) / start.elapsed().as_secs_f64()
            );
        }
    }
    let elapsed = start.elapsed();
    println!("cnr-2000 took: {:?}", elapsed);
    println!("edges/sec: {:.3}", 3216152.0 / elapsed.as_secs_f64());
}