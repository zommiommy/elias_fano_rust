//! Read sequentially clueweb 12 and compare it against the ascii graph for 
//! correctness

use std::fs::File;
use std::io::{self, BufRead};

use elias_fano_rust::prelude::*;
use std::time::Instant;

/// all graph base names sorted by file size
/// ls -aSr | grep ".graph$" | cut -d "." -f 1 -
const BASENAMES: &[&str] = &[
    "wordassociation-2011",
    "enron",
    "uk-2007-05@100000",
    "cnr-2000",
    "dblp-2010",
    "in-2004",
    "amazon-2008",
    "dblp-2011",
    "uk-2007-05@1000000",
    "eu-2005",
    "imdb-2021",
    "uk-2014-tpd",
    "eswiki-2013",
    "indochina-2004",
    "itwiki-2013",
    "uk-2014-host",
    "frwiki-2013",
    "dewiki-2013",
    "hollywood-2009",
    "uk-2002",
    "eu-2015-tpd",
    "ljournal-2008",
    "hollywood-2011",
    "arabic-2005",
    "eu-2015-host",
    "enwiki-2013",
    "enwiki-2015",
    "enwiki-2016",
    "enwiki-2017",
    "uk-2005",
    "enwiki-2018",
    "enwiki-2019",
    "enwiki-2020",
    "enwiki-2021",
    "it-2004",
    "enwiki-2022",
    "webbase-2001",
    "sk-2005",
    "uk-2006-06",
    "uk-2006-10",
    "gsh-2015-tpd",
    "uk-2006-07",
    "uk-2006-05",
    "uk-2006-12",
    "uk-2006-08",
    "uk-2006-11",
    "uk-2007-05",
    "uk-2007-04",
    "uk-2006-09",
    "uk-2007-03",
    "uk-2007-01",
    "uk-2007-02",
    "gsh-2015-host",
    "twitter-2010",
    "uk-2014",
    "gsh-2015",
    "clueweb12",
    "eu-2015",    
];

fn test_graph(basename: &str) {
    let start = Instant::now();

    let wg = WebGraph::<_, 8>::new(format!("/bfd/webgraph/{}",&basename)).unwrap();
    let elapsed = start.elapsed();
    println!("loading {} took: {:?}", &basename, elapsed);

    let truth_path = format!("/bfd/webgraph/{}_ascii.graph-txt", &basename);
    let file = File::open(&truth_path).expect(&format!("Cannot open file {}", truth_path));
    let mut lines = io::BufReader::with_capacity(1<<20, file).lines().skip(1);

    let mut edges = 0;
    let mut old_offset = 0;

    for node_id in 0..wg.properties.nodes {
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
                100.0 * (edges as f64 / wg.properties.arcs as f64), 
                (node_id as f64 / 1_000_000.0) / delta,
                eps / 1_000_000.0,
                ((wg.properties.arcs - edges) as f64 / eps) / 60.0,
                delta / 60.0,
            );
        }
    }
    let elapsed = start.elapsed();
    println!("{} took: {:?}", basename, elapsed);
    println!("edges/sec: {:.3}", wg.properties.arcs as f64 / elapsed.as_secs_f64());
    println!("nodes/sec: {:.3}", wg.properties.nodes as f64 / elapsed.as_secs_f64());
    println!("eges encountered {:.3}", edges);
}

fn main() {
    for (i, basename) in BASENAMES.iter().enumerate() {
        println!("{} / {} - {}", 1 + i, BASENAMES.len(), basename);
        test_graph(basename);
    }
}
