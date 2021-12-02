mod utils;
use utils::*;

use elias_fano_rust::prelude::*;
use std::time::Instant;

const EDGES: usize = 42_574_107_469;
const NODES: usize = 978_408_098;

#[test]
/// Check that elias fano runs considering a lot of possible combinations.
fn test_clueweb12() {
    let start = Instant::now();
    // memory map the test graph file
    let mmap = MemoryMappedFileReadOnly::open(
        "/home/zommiommy/Downloads/clueweb12.graph"
    ).unwrap();

    // create a backend that reads codes from the MSB to the LSb
    let backend =  BitArrayM2L::new(mmap);

    let mut wg = WebGraph::new(
        ConstWebGraphReader::<_, _>::new(&backend),
        vec![0],
    );

    let mut edges = 0;
    for node_id in 0..NODES {
        let (offset, neighbours) = 
            wg.get_neighbours(node_id).unwrap();

        wg.push_offset(offset);
        
        edges += neighbours.len();
        if (node_id & 0xffff) == 0 {
            let delta = start.elapsed().as_secs_f64();
            let eps = edges as f64 / delta;
            println!(
                "[{:.3}%] [{:.3} M nodes/sec]  [{:.3} M edges/sec] [{:.3} ETA minutes] [{:.3} Elapsed minutes]", 
                100.0 * (edges as f64 / EDGES as f64), 
                (node_id as f64 / 1_000_000.0) / delta,
                eps / 1_000_000.0,
                ((EDGES - edges) as f64 / eps) / 60.0,
                delta / 60.0,
            );
        }
    }
    let elapsed = start.elapsed();
    println!("clueweb12 took: {:?}", elapsed);
    println!("edges/sec: {:.3}", EDGES as f64 / elapsed.as_secs_f64());
    println!("nodes/sec: {:.3}", NODES as f64 / elapsed.as_secs_f64());
    println!("eges encountered {:.3}", edges);
}