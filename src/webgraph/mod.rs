#![allow(missing_docs)]
use crate::codes::*;
use crate::traits::*;
use crate::utils::*;
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

const DEBUG: bool = false;

#[macro_export]
macro_rules! debug {
    ($($token:tt)*) => {{
        if DEBUG {
            dbg!($($token)*)
        } else {
            $($token)*
        }
    }};
}

/// Read only WebGraph
pub struct WebGraph<
    Backend: WebGraphReader<WebGraphReaderType>,
    WebGraphReaderType: WebGraphReaderBackend,
> {
    /// The codes.
    pub backend: Backend,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    pub nodes_index: Vec<usize>,
    
    min_interval_length: usize,

    /// make the compiler happy
    _marker: core::marker::PhantomData<WebGraphReaderType>,
}


impl<Backend, WebGraphReaderType> WebGraph<Backend, WebGraphReaderType>
where
    WebGraphReaderType: WebGraphReaderBackend,
    Backend: WebGraphReader<WebGraphReaderType>,
{
    pub fn new(backend: Backend, nodes_index: Vec<usize>) -> Self {
        WebGraph {
            backend,
            nodes_index,
            min_interval_length: 4, // TODO!: Expose
            _marker: core::marker::PhantomData::default(),
        }
    }

    pub fn push_offset(&mut self, offset: usize) {
        self.nodes_index.push(offset);
    }

    pub fn get_last_offset(&self) -> usize {
        *self.nodes_index.last().unwrap()
    }

    /// Get the degree of a given node
    pub fn get_degree(&'a self, node_id: usize) -> Result<usize> {
        let mut reader = (&self.backend)
            .get_reader(self.nodes_index[node_id as usize] as usize);
        reader.read_outdegree()
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&'a self, node_id: usize) -> Result<(usize, Vec<usize>)> {

        // move to the node data
        let index = self.nodes_index[node_id];

        let mut neighbours = Vec::new();

        let mut reader = (&self.backend).get_reader(index);
        // read the degree
        let degree = debug!(reader.read_outdegree()?);
        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            return Ok((reader.tell_bits()?, vec![]));
        }

        // actually decode the neighbours
        neighbours = self.decode_references(&mut reader, node_id, neighbours)?;
        neighbours = self.dencode_extra_nodes(&mut reader, node_id, degree, neighbours)?;

        // TODO!: if we do a proper merge of intervalizzation and residuals we 
        // can avoid the sorting
        neighbours.sort_unstable();

        Ok((reader.tell_bits()?, neighbours))
    }

    #[inline]
    fn decode_references(
        &'a self,
        reader: &mut WebGraphReaderType,
        node_id: usize,
        mut neighbours: Vec<usize>,
    ) -> Result<Vec<usize>> {
        // figure out the ref
        let ref_delta = debug!(reader.read_reference_offset()?);
        if ref_delta == 0 {
            return Ok(neighbours);
        }

        // compute which node we are refering to
        let ref_node = node_id - ref_delta;

        // Copy blocks, decode the run length encoding, and then
        // proceed as the copy list read
        let number_of_blocks = debug!(reader.read_block_count()?);

        // if there are no block -> we copy all the neighbours
        if number_of_blocks == 0 {
            neighbours.extend(&self.get_neighbours(ref_node)?.1);
            return Ok(neighbours);
        }
        // decode the run-length copy blocks
        let mut blocks = vec![debug!(reader.read_blocks()?)];
        for _ in 0..number_of_blocks.saturating_sub(1) {
            blocks.push(debug!(reader.read_blocks()? + 1));
        }

        // recursive call to decode its neighbours
        let ref_neighbours = self.get_neighbours(ref_node)?;

        let mut curr_bit_value = true;
        let mut blocks_iter = blocks.iter();
        let mut counter = *blocks_iter.next().unwrap();
        for node in ref_neighbours.1 {
            if counter == 0 {
                curr_bit_value ^= true;
                counter = *blocks_iter.next().unwrap_or(&usize::MAX);
            }

            if curr_bit_value && counter > 0 {
                neighbours.push(node);
            }

            counter -= 1;
        }
        
        Ok(neighbours)
    }

    #[inline]
    /// Dencode the list of extra nodes as deltas using zeta3 codes.
    fn dencode_extra_nodes(
        &self,
        reader: &mut WebGraphReaderType,
        node_id: usize,
        degree: usize,
        mut neighbours: Vec<usize>,
    ) -> Result<Vec<usize>> {
        let mut nodes_to_decode = degree - neighbours.len() as usize;
        // early stop
        if nodes_to_decode == 0 {
            return Ok(neighbours);
        }
        let interval_count = debug!(reader.read_interval_count()?);
        if interval_count > 0 {

            let mut start = (nat2int(debug!(reader.read_interval_start()?)) 
                + node_id as isize) as usize;

            let mut delta = debug!(reader.read_interval_len()?) 
                + self.min_interval_length;

            neighbours.extend(start..start + delta);
            start += delta;
            nodes_to_decode -= delta;

            for _ in 0..interval_count.saturating_sub(1) {
                start += debug!(reader.read_interval_start()?) + 1;
                delta = debug!(reader.read_interval_len()?) + self.min_interval_length;

                neighbours.extend(start..start + delta);
                
                start += delta;
                nodes_to_decode -= delta;
            }
        }

        // early stop if all the neighbours were in intervals
        if nodes_to_decode == 0 {
            return Ok(neighbours);
        }

        // read the first neighbour
        let first_neighbour_delta = debug!(reader.read_first_residual()?);
        // decode the first neighbour
        let first_neighbour = ((node_id as isize) 
            + debug!(nat2int(first_neighbour_delta))) as usize;
        neighbours.push(first_neighbour);

        // decode the other extra nodes
        let mut tmp = first_neighbour;
        for _ in 0..nodes_to_decode.saturating_sub(1) {
            let new_node = debug!(debug!(reader.read_residual()?) + tmp + 1);
            neighbours.push(new_node);
            tmp = new_node;
        }
        Ok(neighbours)
    }
}
