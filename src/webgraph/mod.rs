use crate::BitStream;
use crate::compact_array::CompactArray;

mod graph_utils;
mod builder;
pub use builder::*;

pub struct WebGraph {
    /// The codes.
    /// For each node we are going to write its encoded degree,
    /// the first neighbour, and then the encoded gaps between neighbours.
    data: BitStream,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    nodes_index: CompactArray,
}

// impl GraphUtils for WebGraph {
//     /// Get the degree of a node
//     fn degree(&self, node_id: u64) -> u64 {
//         self.data.seek(self.nodes_index.read(src));
//         self.data.read_gamma()
//     }
// 
//     fn iter_neighbours(&self, src: u64) {
//         self.data.seek(self.nodes_index.read(src));
//         let degree = self.data.read_gamma();
// 
//         let mut tmp = src;
//         for _ in 0..degree {
//             
//         }
//     }
// }

impl crate::traits::MemoryFootprint for WebGraph {
    fn total_size(&self) -> usize {
        self.data.total_size() + self.nodes_index.total_size()
    }
}
