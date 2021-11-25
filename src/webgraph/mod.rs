#![allow(missing_docs)]
use crate::codes::*;
use crate::compact_array::CompactArray;
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

mod builder;
pub use builder::*;

// add a layer of indirection
// backend that writes code so we can support both compile time and runtime shit

/// Webgraph by Sebastiano Vigna and Paolo Boldi
pub struct WebGraphWriter<BACKEND: WebGraphWriterBackend> {
    /// The codes.
    data: BACKEND,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    nodes_index: Vec<usize>,
}

/// Webgraph by Sebastiano Vigna and Paolo Boldi
pub struct WebGraphReader<BACKEND: WebGraphReaderBackend> {
    /// The codes.
    data: BACKEND,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    nodes_index: Vec<usize>,
}

impl<BACKEND: WebGraphBackend> WebGraph<BACKEND> {
    pub fn new(backend: BACKEND) -> WebGraph<BACKEND> {
        WebGraph {
            data: backend,
            nodes_index: Vec::new(),
        }
    }

    /// Get the degree of a given node
    pub fn get_degree(&mut self, node_id: usize) -> Result<usize> {
        let old_pos = self.data.tell_bits()?;
        self.data
            .seek_bits(self.nodes_index[node_id as usize] as usize)?;
        let res = self.data.read_gamma()?;
        self.data.seek_bits(old_pos)?;
        Ok(res)
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&mut self, node_id: usize) -> Result<Vec<usize>> {
        let mut neighbours = Vec::new();

        // backup the position so we can reset it later
        let old_pos = self.data.tell_bits()?;

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
        self.data.seek_bits(index as usize)?;
        // read the degree
        let degree = self.data.read_gamma()?;
        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            self.data.seek_bits(old_pos)?;
            return Ok(vec![]);
        }

        // actually decode the neighbours
        if USE_REFERENCES {
            self.decode_references(node_id, &mut neighbours)?;
        }
        self.dencode_extra_nodes(node_id, degree, &mut neighbours)?;

        // reset the reader to where it was
        self.data.seek_bits(old_pos)?;

        Ok(neighbours)
    }

    #[inline]
    fn decode_references(&mut self, node_id: usize, neighbours: &mut Vec<usize>) -> Result<()> {
        // figure out the ref
        let ref_delta = self.data.read_gamma()?;
        if ref_delta == 0 {
            return Ok(());
        }

        // compute which node we are refering to
        let ref_node = node_id - ref_delta;
        // recursive call to decode its neighbours
        let ref_neighbours = self.get_neighbours(ref_node)?;
        if USE_COPY_LIST {
            // add the nodes to be copied
            for node in ref_neighbours {
                if self.data.read_bit()? {
                    neighbours.push(node);
                }
            }
        } else {
            // Copy blocks, decode the run length encoding, and then
            // proceed as the copy list read
            let number_of_blocks = self.data.read_gamma()?;
            let mut blocks = vec![self.data.read_gamma()?];
            for _ in 0..number_of_blocks {
                blocks.push(self.data.read_gamma()? + 1);
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
        Ok(())
    }

    #[inline]
    /// Dencode the list of extra nodes as deltas using zeta3 codes.
    fn dencode_extra_nodes(
        &mut self,
        node_id: usize,
        degree: usize,
        neighbours: &mut Vec<usize>,
    ) -> Result<()> {
        let nodes_to_decode = degree - neighbours.len() as usize;
        // early stop
        if nodes_to_decode == 0 {
            return Ok(());
        }

        if USE_INTERVALIZZATION {
            let number_of_ranges = self.data.read_gamma()?;

            for _ in 0..number_of_ranges {
                let start = self.data.read_gamma()?;
                let delta = self.data.read_gamma()?;
                neighbours.extend(start..start + delta);
            }
        }

        // read the first neighbour
        let first_neighbour_delta = self.data.read_gamma()?;
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
            let new_node = self.data.read_zeta::<3>()? + tmp;
            neighbours.push(new_node);
            tmp = new_node;
        }
        Ok(())
    }
}

impl<BACKEND: WebGraphBackend> crate::traits::MemoryFootprint for WebGraph<BACKEND> {
    fn total_size(&self) -> usize {
        self.data.total_size() + self.nodes_index.total_size()
    }
}
