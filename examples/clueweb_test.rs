//! Read sequentially clueweb 12 and compare it against the ascii graph for 
//! correctness

use std::fs::File;
use std::io::{self, BufRead};

use elias_fano_rust::prelude::*;
use std::time::Instant;

const EDGES: usize = 42_574_107_469;
const NODES: usize = 978_408_098;

fn main() {
    let start = Instant::now();

    let wg = WebGraph::<_, 8>::new("/bfd/clueweb12").unwrap();
    let elapsed = start.elapsed();
    println!("loading clueweb12 took: {:?}", elapsed);

    let file = File::open("/bfd/clueweb12_ascii.graph-txt").unwrap();
    let mut lines = io::BufReader::with_capacity(1<<20, file).lines().skip(1);

    let mut old_offset = 0;
    let mut edges = 0;

    for node_id in 0..NODES {
        let truth = lines.next().unwrap().unwrap()
            .split(" ")
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

        assert_eq!(truth.len(), wg.get_degree(node_id).unwrap(), "The degree of the node '{}' does not match the truth", node_id);
        let (new_offset, neighbours) = wg.get_neighbours_and_offset(node_id).unwrap();
        assert_eq!(old_offset, wg.offsets.get(node_id).unwrap(), "The offsets for node id '{}' do not match", node_id);
        old_offset = new_offset;

        assert_eq!(truth, neighbours, "The neighbours of node id '{}' don't match the truth. Its offset is: '{}'", node_id, old_offset);

        assert_eq!(truth, wg.iter_neighbours(node_id).unwrap().collect::<Vec<_>>(), "The iter neighbours of node id '{}' don't match the truth. Its offset is: '{}'", node_id, old_offset);

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
