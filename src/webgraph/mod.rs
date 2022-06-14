#![allow(missing_docs)]
use crate::codes::*;
use crate::traits::*;
use crate::utils::*;
use crate::Result;
use core::iter::Peekable;
use core::intrinsics::unlikely;

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
pub struct WebGraph<Backend: WebGraphReader> {
    /// The codes.
    pub backend: Backend,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    pub nodes_index: Vec<usize>,
    
    min_interval_length: usize,
}

pub struct MaskedIterator<'a, Backend: WebGraphReader> {
    /// The resolved reference node, if present
    parent: Box<WebGraphLazyIter<'a, Backend>>,
    /// The copy blocks from the ref node
    blocks: Vec<usize>,
    /// The id of block to parse
    block_idx: usize,
    /// Caching of the number of values returned, if needed
    size: usize,
}

impl<'a, Backend> MaskedIterator<'a, Backend>
where 
    Backend: WebGraphReader 
{

    pub fn new(parent: WebGraphLazyIter<'a, Backend>, blocks: Vec<usize>) 
        -> Self {
        // compute the number of nodes to copy 
        let size: usize = blocks.iter().enumerate()
            .filter(|(i, x)| i & 1 == 0)
            .map(|(_, x)| x)
            .sum();

        Self {
            parent: Box::new(parent),
            blocks,
            block_idx: 0,
            size,
        }
    }
}

impl<'a, Backend> Iterator for MaskedIterator<'a, Backend>
where 
    Backend: WebGraphReader 
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.block_idx <= self.blocks.len());
        // no more copy blocks so we can stop the parsing
        if unlikely(self.blocks.len() == self.block_idx) {
            return None;
        }

        let mut current_block = self.blocks[self.block_idx];
        // we finished this block so we must skip the next one, if present
        if unlikely(current_block == 0) {
            // skip the next block
            self.block_idx += 1;
            debug_assert!(self.blocks[self.block_idx] > 0);
            for _ in 0..self.blocks[self.block_idx] {
                // should we add `?` and do an early return? 
                // I don't think it improves speed because it add an 
                // unpredictable branch and the blocks should be done so that
                // they are always right.
                let node = self.parent.next(); 
                debug_assert!(node.is_some());
            }
            self.block_idx += 1;
            current_block = self.blocks[self.block_idx];
            debug_assert_ne!(current_block, 0);
        }

        let result = self.parent.next();
        self.blocks[self.block_idx] -= 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

pub struct WebGraphLazyIter<'a, Backend: WebGraphReader>{
    /// The degree of the current node
    degree: usize,

    copied_nodes_iter: Option<Peekable<MaskedIterator<'a, Backend>>>,

    /// Intervals of extra nodes
    intervals: Vec<(usize, usize)>,
    intervals_idx: usize,
    /// Extra nodes
    extra_nodes: Vec<usize>,
    extra_nodes_idx: usize,
}

impl<'a, Backend> Iterator for WebGraphLazyIter<'a, Backend>
where 
    Backend: WebGraphReader 
{
    type Item = usize;
    /// In this iteration we have to do a 3-way merge, this implies that there 
    /// are 3! = 6 possible cases (we can ignore equalities), 
    /// to reduce the complexity, we prefer finding the minimum which can be done
    /// branchless and then just comparing the min with the values so we can have
    /// 3 branches, which should reduce code duplication (better instructions 
    /// cache) and less missing branches. 
    fn next(&mut self) -> Option<Self::Item> {
        // check if we should stop iterating
        if self.degree == 0 {
            return None;
        }

        self.degree -= 1;

        // Get the different nodes or usize::MAX if not present
        let copied_value = *self.copied_nodes_iter.as_mut().map(|x| 
            x.peek().unwrap_or(&usize::MAX)
        ).unwrap_or(&usize::MAX);

        let extra_node = *self.extra_nodes.get(self.extra_nodes_idx)
            .unwrap_or(&usize::MAX);

        let interval_node = *{
            let (start, len) = self.intervals.get(self.intervals_idx)
                .unwrap_or(&(usize::MAX, usize::MAX));
            debug_assert_ne!(*len, 0, "there should never be an interval with length zero here");
            start
        };

        debug_assert!(
            copied_value != usize::MAX 
            ||
            extra_node != usize::MAX
            ||
            interval_node != usize::MAX,
            "At least one of the nodes must present, this should be a problem with the degree.",
        );

        // find the smallest of the values
        let min = copied_value.min(extra_node).min(interval_node);

        // depending on from where the node was, forward it
        if min == copied_value {
            self.copied_nodes_iter.as_mut().unwrap().next().unwrap();
        } else if min == extra_node {
            self.extra_nodes_idx += 1;
        } else {
            let (start, len) = &mut self.intervals[self.intervals_idx];
            debug_assert_ne!(*len, 0, "there should never be an interval with length zero here");
            // if the interval has other values, just reduce the interval
            if *len > 1 {
                *len -= 1;
                *start += 1;
            } else {
                // otherwise just increase the idx to use the next interval
                self.intervals_idx += 1;
            }
        }

        Some(min)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.degree, Some(self.degree)) 
    }
}

impl<'a, Backend> WebGraphLazyIter<'a, Backend> 
where 
    Backend: WebGraphReader 
{
    /// Create an empty iterator
    pub fn empty() -> Self {
        Self {
            degree: 0,
            copied_nodes_iter: None,
            intervals: vec![],
            intervals_idx: 0,
            extra_nodes: vec![],
            extra_nodes_idx: 0,
        }
    }

    pub fn new(reference: &'a WebGraph<Backend>, node_id: usize) -> Result<Self> {
        // get a stream reader for the current node 
        let mut reader = (&reference.backend).get_reader(reference.nodes_index[node_id]);

        let degree = reader.read_outdegree()?;       
        if degree == 0 {
            // Empty iterator
            return Ok(Self::empty());
        }
        let mut nodes_left_to_decode = degree;
        
        // decode the reference
        let ref_delta = reader.read_reference_offset()?;
        let copied_nodes_iter = if ref_delta == 0 {
            None
        } else {
            let number_of_blocks = reader.read_block_count()?;
            let ref_node = node_id - ref_delta;
            
            let blocks = if number_of_blocks == 0 {
                vec![usize::MAX]
            } else {
                let mut blocks = Vec::with_capacity(number_of_blocks);
                blocks.push(reader.read_blocks()?);
                for _ in 0..number_of_blocks.saturating_sub(1) {
                    blocks.push(reader.read_blocks()? + 1);
                }
                blocks
            };

            let res = MaskedIterator::new(WebGraphLazyIter::new(reference, ref_node)?, blocks);
            debug_assert!(nodes_left_to_decode >= res.size);
            nodes_left_to_decode -= res.size;
            Some(res.peekable())
        };

        // decode intervals
        let intervals = if nodes_left_to_decode == 0 {
            vec![]
        } else {
            let number_of_intervals = reader.read_interval_count()?;
            // decode the intervals if present
            if number_of_intervals == 0 {
                vec![]
            } else {
                let mut intervals = Vec::with_capacity(number_of_intervals);
                let mut start = (nat2int(reader.read_interval_start()?) as isize + node_id as isize) as usize;
                let mut delta = reader.read_interval_len()? + reference.min_interval_length;
        
                intervals.push((start, delta));
                start += delta;
                nodes_left_to_decode -= delta;

                for _ in 0..number_of_intervals.saturating_sub(1) {
                    start += reader.read_interval_start()? + 1;
                    delta = reader.read_interval_len()? + reference.min_interval_length;

                    intervals.push((start, delta));
                    start += delta;
                    nodes_left_to_decode -= delta;
                }
                intervals
            }
        };
       
        // decode the extra nodes
        let extra_nodes = if nodes_left_to_decode == 0 {
            vec![]
        } else {
            let mut extra_nodes = Vec::with_capacity(nodes_left_to_decode);
            let mut tmp = (
                node_id as isize + nat2int(reader.read_first_residual()?)    
            ) as usize;
            extra_nodes.push(tmp);

            for _ in 0..nodes_left_to_decode.saturating_sub(1) {
                tmp = reader.read_residual()? + tmp + 1;
                extra_nodes.push(tmp);
            }
            extra_nodes
        };

        Ok(Self {
            degree,
            copied_nodes_iter,
            intervals,
            intervals_idx: 0,
            extra_nodes,
            extra_nodes_idx: 0,
        })
    }
}

impl<Backend: WebGraphReader> WebGraph<Backend> {
    pub fn new(backend: Backend, nodes_index: Vec<usize>) -> Self {
        WebGraph {
            backend,
            nodes_index,
            min_interval_length: 4, // TODO!: Expose
        }
    }

    pub fn push_offset(&mut self, offset: usize) {
        self.nodes_index.push(offset);
    }

    pub fn get_last_offset(&self) -> usize {
        *self.nodes_index.last().unwrap()
    }

    /// Get the degree of a given node
    pub fn get_degree(&self, node_id: usize) -> Result<usize> {
        let mut reader = (&self.backend)
            .get_reader(self.nodes_index[node_id as usize] as usize);
        reader.read_outdegree()
    }

    /// Get the neighbours of a given node
    pub fn iter_neighbours(&self, node_id: usize) -> Result<WebGraphLazyIter<'_, Backend>> {
        WebGraphLazyIter::new(self, node_id)
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&self, node_id: usize) -> Result<(usize, Vec<usize>)> {

        // move to the node data
        let index = self.nodes_index[node_id];

        let mut reader = (&self.backend).get_reader(index);
        // read the degree
        let degree = debug!(reader.read_outdegree()?);
        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            return Ok((reader.tell_bits()?, vec![]));
        }

        // actually decode the neighbours
        let copied_neighbours = self.decode_references(&mut reader, node_id)?;
        let (intervals, extra_nodes) = 
            self.dencode_extra_nodes(&mut reader, node_id, 
                degree - copied_neighbours.len()
        )?;

        let neighbours = three_way_merge(copied_neighbours, intervals, extra_nodes);

        Ok((reader.tell_bits()?, neighbours))
    }

    #[inline]
    fn decode_references(
        &self,
        reader: &mut Backend::WebGraphReaderType,
        node_id: usize,
    ) -> Result<Vec<usize>> {
        let mut copied_neighbours = vec![];
        // figure out the ref
        let ref_delta = debug!(reader.read_reference_offset()?);
        if ref_delta == 0 {
            return Ok(copied_neighbours);
        }

        // compute which node we are refering to
        let ref_node = node_id - ref_delta;

        // Copy blocks, decode the run length encoding, and then
        // proceed as the copy list read
        let number_of_blocks = debug!(reader.read_block_count()?);

        // if there are no block -> we copy all the neighbours
        if number_of_blocks == 0 {
            copied_neighbours.extend(&self.get_neighbours(ref_node)?.1);
            return Ok(copied_neighbours);
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
                copied_neighbours.push(node);
            }

            counter -= 1;
        }
        
        Ok(copied_neighbours)
    }

    #[inline]
    /// Dencode the list of extra nodes as deltas using zeta3 codes.
    fn dencode_extra_nodes(
        &self,
        reader: &mut Backend::WebGraphReaderType,
        node_id: usize,
        mut nodes_left_to_decode: usize,
    ) -> Result<(Vec<usize>, Vec<usize>)> {
        let mut interval_nodes = vec![];
        let mut extra_nodes = vec![];
        // early stop
        if nodes_left_to_decode == 0 {
            return Ok((interval_nodes, extra_nodes));
        }
        let interval_count = debug!(reader.read_interval_count()?);
        if interval_count > 0 {

            let mut start = (nat2int(debug!(reader.read_interval_start()?)) 
                + node_id as isize) as usize;

            let mut delta = debug!(reader.read_interval_len()?) 
                + self.min_interval_length;

            interval_nodes.extend(start..start + delta);
            start += delta;
            nodes_left_to_decode -= delta;

            for _ in 0..interval_count.saturating_sub(1) {
                start += debug!(reader.read_interval_start()?) + 1;
                delta = debug!(reader.read_interval_len()?) + self.min_interval_length;

                interval_nodes.extend(start..start + delta);
                
                start += delta;
                nodes_left_to_decode -= delta;
            }
        }

        // early stop if all the neighbours were in intervals
        if nodes_left_to_decode == 0 {
            return Ok((interval_nodes, extra_nodes));
        }

        // read the first neighbour
        let first_neighbour_delta = debug!(reader.read_first_residual()?);
        // decode the first neighbour
        let first_neighbour = ((node_id as isize) 
            + debug!(nat2int(first_neighbour_delta))) as usize;
        extra_nodes.push(first_neighbour);

        // decode the other extra nodes
        let mut tmp = first_neighbour;
        for _ in 0..nodes_left_to_decode.saturating_sub(1) {
            let new_node = debug!(debug!(reader.read_residual()?) + tmp + 1);
            extra_nodes.push(new_node);
            tmp = new_node;
        }
        Ok((interval_nodes, extra_nodes))
    }
}

#[inline]
fn three_way_merge(first: Vec<usize>, second: Vec<usize>, third: Vec<usize>) -> Vec<usize> {
    let mut result = Vec::with_capacity(first.len() + second.len() + third.len());
    let mut first_i = 0;
    let mut second_i = 0;
    let mut third_i = 0;

    loop {
        match (first.get(first_i), second.get(second_i), third.get(third_i)) {
            (None, None, None) => {
                return result;
            }
            (f, s, t) => {
                let f = f.copied().unwrap_or(usize::MAX);
                let s = s.copied().unwrap_or(usize::MAX);
                let t = t.copied().unwrap_or(usize::MAX);

                if f < s {
                    if t < f {
                        result.push(t);
                        third_i += 1;
                    } else {
                        result.push(f);
                        first_i += 1;
                    }
                } else {
                    if t < s {
                        result.push(t);
                        third_i += 1;
                    } else {
                        result.push(s);
                        second_i += 1;
                    }
                }
            }
        }
    }
}
