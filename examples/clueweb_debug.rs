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

    let truth = vec![626916273, 928759358, 932543914, 932543917, 932544706, 932544707, 932544708, 932544932, 932544933, 932544934, 932544936, 932544937, 932544939, 932544942];

    let neighbours = wg.get_neighbours(932544940).unwrap();

    assert_eq!(truth, neighbours);
    assert_eq!(truth, wg.iter_neighbours(932544940).unwrap().collect::<Vec<_>>());
}
