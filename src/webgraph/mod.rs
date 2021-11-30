#![allow(missing_docs)]
use crate::codes::*;
use crate::traits::*;
use crate::Result;

mod traits;
pub use traits::*;
mod runtime_webgraph_backend;
pub use runtime_webgraph_backend::*;
mod const_webgraph_backend;
pub use const_webgraph_backend::*;

mod bv_compatability;
pub use bv_compatability::*;

//mod builder;
//pub use builder::*;

/// FLAG if the struct should use differential compression or not.
pub const USE_REFERENCES: bool = true;
/// Maximum depth of references, lower values will result in faster
/// decompression but worse compression
pub const MAX_REFERENCE_RECURSION: usize = 128;
/// Maximum distance between the current node and the reference one
/// an higher value can result in better compression but slows down
/// the reference finding algorithm, so the compression will be slower
pub const MAX_REFERENCE_DISTANCE: usize = 1 << 10;
/// Minimum score that a neighbour has to have to be a reference
pub const MIN_SCORE_THRESHOLD: f64 = 1.0;
/// If we should use a bitmap for the copy list or
/// the runlength encoding of it
pub const USE_COPY_LIST: bool = false;
/// If the extra nodes shold be encoded as dense ranges and residuals
/// or just deltas
pub const USE_INTERVALIZZATION: bool = true;
pub const MIN_INTERVALIZZATION_LEN: usize = 3;
// If during the compression we should employ the swap_remove trick
// which requires then sorting the current_dsts possibly multiple times
// TODO!: Finish validating this
pub const SWAP_REMOVE: bool = false;

/// Read only WebGraph
pub struct WebGraph<'a, BACKEND: WebGraphReader<'a> + MemoryFootprint> {
    /// The codes.
    backend: BACKEND,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    nodes_index: Vec<usize>,

    /// Make the compiler happy
    phantom: std::marker::PhantomData<&'a [()]>,
}

impl<'a, BACKEND> MemoryFootprint for WebGraph<'a, BACKEND>
where
    BACKEND: WebGraphReader<'a> + MemoryFootprint,
{
    fn total_size(&self) -> usize {
        self.backend.total_size() + self.nodes_index.total_size()
    }
}

impl<'a, BACKEND> WebGraph<'a, BACKEND>
where
    BACKEND: WebGraphReader<'a> + MemoryFootprint,
{
    pub fn new(backend: BACKEND, nodes_index: Vec<usize>) -> WebGraph<'a, BACKEND> {
        WebGraph {
            backend,
            nodes_index,
            phantom: std::marker::PhantomData::default(),
        }
    }

    /// Get the degree of a given node
    pub fn get_degree(&'a self, node_id: usize) -> Result<usize> {
        let mut reader = self
            .backend
            .get_reader(self.nodes_index[node_id as usize] as usize);
        reader.read_outdegree()
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&'a self, node_id: usize) -> Result<Vec<usize>> {
        let mut neighbours = Vec::new();

        // move to the node data
        let index = self.nodes_index[node_id];
        // if the index is the same as the next one, then
        // we have no stuff to read
        // also, if it's the last one we know that it's not empty
        if self
            .nodes_index
            .get(1 + node_id)
            .map(|x| index == *x)
            .unwrap_or(false)
        {
            return Ok(vec![]);
        }

        let mut reader = self.backend.get_reader(index);
        // read the degree
        let degree = reader.read_outdegree()?;
        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            return Ok(vec![]);
        }

        // actually decode the neighbours
        if USE_REFERENCES {
            neighbours = self.decode_references(&mut reader, node_id, neighbours)?;
        }
        neighbours = self.dencode_extra_nodes(&mut reader, node_id, degree, neighbours)?;

        Ok(neighbours)
    }

    #[inline]
    fn decode_references(
        &'a self,
        reader: &mut BACKEND::ReaderType,
        node_id: usize,
        mut neighbours: Vec<usize>,
    ) -> Result<Vec<usize>> {
        // figure out the ref
        let ref_delta = reader.read_reference_offset()?;
        if ref_delta == 0 {
            return Ok(neighbours);
        }

        // compute which node we are refering to
        let ref_node = node_id - ref_delta;
        // recursive call to decode its neighbours
        let ref_neighbours = self.get_neighbours(ref_node)?;
        if USE_COPY_LIST {
            // add the nodes to be copied
            for node in ref_neighbours {
                if reader.read_bit()? {
                    neighbours.push(node);
                }
            }
        } else {
            // Copy blocks, decode the run length encoding, and then
            // proceed as the copy list read
            let number_of_blocks = reader.read_block_count()?;
            let mut blocks = vec![reader.read_blocks()?];
            for _ in 0..number_of_blocks {
                blocks.push(reader.read_blocks()? + 1);
            }

            let mut curr_bit_value = true;
            let mut blocks_iter = blocks.iter();
            let mut counter = *blocks_iter.next().unwrap();
            for node in ref_neighbours {
                if counter == 0 {
                    curr_bit_value ^= true;
                    counter = *blocks_iter.next().unwrap_or(&usize::MAX);
                }

                if curr_bit_value && counter > 0 {
                    neighbours.push(node);
                }

                counter -= 1;
            }
        }
        Ok(neighbours)
    }

    #[inline]
    /// Dencode the list of extra nodes as deltas using zeta3 codes.
    fn dencode_extra_nodes(
        &self,
        reader: &mut BACKEND::ReaderType,
        node_id: usize,
        degree: usize,
        mut neighbours: Vec<usize>,
    ) -> Result<Vec<usize>> {
        let nodes_to_decode = degree - neighbours.len() as usize;
        // early stop
        if nodes_to_decode == 0 {
            return Ok(neighbours);
        }

        if USE_INTERVALIZZATION {
            let number_of_ranges = reader.read_interval_count()?;

            for _ in 0..number_of_ranges {
                let start = reader.read_interval_start()?;
                let delta = reader.read_interval_len()?;
                neighbours.extend(start..start + delta);
            }
        }

        // read the first neighbour
        let first_neighbour_delta = reader.read_first_residual()?;
        // decode the first neighbour
        let first_neighbour = if first_neighbour_delta & 1 == 0 {
            node_id + (first_neighbour_delta >> 1)
        } else {
            node_id - (first_neighbour_delta >> 1) - 1
        };
        neighbours.push(first_neighbour);

        // decode the other extra nodes
        let mut tmp = first_neighbour;
        for _ in 0..nodes_to_decode - 1 {
            let new_node = reader.read_residual()? + tmp;
            neighbours.push(new_node);
            tmp = new_node;
        }
        Ok(neighbours)
    }
}
