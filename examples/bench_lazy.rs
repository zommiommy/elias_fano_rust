//! Read sequentially clueweb 12 and print performance stats

use elias_fano_rust::prelude::*;
use std::time::Instant;
use std::fs::File;
use std::io::*;
use std::env;

const WARMAP: usize = 3;
const REPEAT: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Please pass a basename, and then a path to a file with a node_idx per line");
        return;
    }

    let basename = args[1].as_str();

    let reader = BufReader::new(File::open(&args[2]).unwrap());
    let nodes = reader.lines().map(|val| 
        val.unwrap().parse::<usize>().unwrap()
    ).collect::<Vec<usize>>();

    let start = Instant::now();
    let wg = WebGraph::<_, 6>::new(format!("/bfd/webgraph/{}", basename)).unwrap();
    let elapsed = start.elapsed();
    println!("loading {} took: {:?}", basename, elapsed);
    
    let mut total_elapsed = 0.0;
    
    let mut edges = 0;
    for node_idx in nodes.iter() {
        edges += wg.get_degree(*node_idx).unwrap();
    }

    let mut z = 0;

    for i in 0..WARMAP + REPEAT {
        let start = Instant::now();
        for node_idx in nodes.iter() {
            z += wg.iter_neighbours(*node_idx).unwrap().count();
        }

        if i >= WARMAP {
            total_elapsed += start.elapsed().as_secs_f64();
        }
    }

    total_elapsed /= REPEAT as f64;

    println!("edges: {:.3}", edges);
    println!("nodes: {:.3}", nodes.len());
    println!("nodes/sec: {:.3}", nodes.len() as f64 / total_elapsed);
    println!("edges/sec: {:.3}", edges as f64 / total_elapsed);
    if z == 0 {
        println!("0");
    }

}