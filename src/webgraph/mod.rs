use crate::compact_array::CompactArray;
use crate::traits::*;
use crate::codes::*;

mod builder;
pub use builder::*;

/// General trait that captures which traits we need for a struct to be a backend
/// of webgraph.
pub trait WebGraphBackend: ReadBit + WriteBit + CodeUnary + CodeFixedLength + MemoryFootprint {}
/// Blanket implementation
impl<T> WebGraphBackend for T 
where
    T: ReadBit + WriteBit + CodeUnary + CodeFixedLength + MemoryFootprint
{}


pub struct WebGraph<BACKEND: WebGraphBackend> {
    /// The codes.
    /// For each node we are going to write its encoded degree,
    /// the first neighbour, and then the encoded gaps between neighbours.
    data: BACKEND,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    nodes_index: CompactArray,
}

// impl GraphUtils for WebGraph {
//     /// Get the degree of a node
//     fn degree(&self, node_id: usize) -> usize {
//         self.data.seek(self.nodes_index.read(src));
//         self.data.read_gamma()
//     }
// 
//     fn iter_neighbours(&self, src: usize) {
//         self.data.seek(self.nodes_index.read(src));
//         let degree = self.data.read_gamma();
// 
//         let mut tmp = src;
//         for _ in 0..degree {
//             
//         }
//     }
// }

impl<BACKEND: WebGraphBackend> crate::traits::MemoryFootprint for WebGraph<BACKEND> {
    fn total_size(&self) -> usize {
        self.data.total_size() + self.nodes_index.total_size()
    }
}
