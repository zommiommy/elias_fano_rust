//! Read sequentially clueweb 12 and print performance stats

use elias_fano_rust::prelude::*;
use std::time::Instant;

const EDGES: usize = 42_574_107_469;
const NODES: usize = 978_408_098;

fn main() {
    let start = Instant::now();

    let wg = WebGraph::<_, 8>::new("/bfd/clueweb12").unwrap();
    let elapsed = start.elapsed();
    println!("loading clueweb12 took: {:?}", elapsed);

    let start = Instant::now();
    let mut edges = 0;
    for node_id in 0..NODES {
        let neighbours = wg.get_neighbours(node_id).unwrap();

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

    let start = Instant::now();
    let mut edges = 0;
    for node_id in 0..NODES {
        edges += wg.iter_neighbours(node_id).unwrap().count();
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