#![allow(missing_docs)]
use crate::codes::*;
use crate::prelude::BitArrayM2L;
use crate::prelude::MemoryMappedFileReadOnly;
use crate::traits::*;
use crate::utils::*;
use crate::Result;
use core::iter::Peekable;
use core::intrinsics::unlikely;
use std::path::Path;

mod offsets;
pub use offsets::*;

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
pub struct WebGraph<Backend: WebGraphReader, const QUANTUM_LOG2: usize=8> {
    pub properties: Properties,

    /// The codes.
    pub backend: Backend,

    /// store the bit-index in the BitStream of each node
    /// Should we use elias-fano here?
    pub offsets: Offsets<QUANTUM_LOG2>,
    
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

    pub fn new(parent: WebGraphLazyIter<'a, Backend>, mut blocks: Vec<usize>, len: usize) 
        -> Self {
        // compute the number of nodes to copy 
        
        // the number of copied nodes
        let mut size: usize = 0;
        // the cumulative sum of the blocks
        let mut cumsum_blocks: usize = 0;
        for (i, x) in blocks.iter().enumerate(){
            // branchless add
            size = if i % 2 == 0{
                size + x
            } else {
                size
            };            
            cumsum_blocks += x;
        }

        // an empty blocks means that we should take all the neighbours
        let remainder = len - cumsum_blocks;
    
        // check if the last block is a copy or skip block
        if remainder != 0 && blocks.len() % 2 == 0 {
            size += remainder;
            blocks.push(remainder);
        }

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
        let mut current_block = self.blocks[self.block_idx];
        // we finished this block so we must skip the next one, if present
        if unlikely(current_block == 0) {
            // skip the next block
            self.block_idx += 1;

            // no more copy blocks so we can stop the parsing
            if unlikely(self.block_idx >= self.blocks.len()) {
                return None;
            }

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

    pub fn new<const QUANTUM_LOG2: usize>(reference: &'a WebGraph<Backend, QUANTUM_LOG2>, node_id: usize) -> Result<Self> {
        // get a stream reader for the current node 
        let offset = reference.offsets.get(node_id)?;
        debug!(node_id);
        debug!(offset);
        let mut reader = (&reference.backend).get_reader(offset);

        let degree = reader.read_outdegree()?;       
        debug!(degree);
        if degree == 0 {
            // Empty iterator
            return Ok(Self::empty());
        }
        let mut nodes_left_to_decode = degree;
        
        // decode the reference
        let ref_delta = reader.read_reference_offset()?;
        debug!(ref_delta);
        let copied_nodes_iter = if ref_delta == 0 {
            None
        } else {
            let number_of_blocks = reader.read_block_count()?;
            let ref_node = node_id - ref_delta;
            debug!(ref_node);
            debug!(number_of_blocks);
            let ref_outdegree = (&reference.backend).get_reader(
                reference.offsets.get(ref_node)?
                ).read_outdegree()?;

            let blocks = if number_of_blocks == 0 {
                vec![
                    ref_outdegree,
                ]
            } else {
                let mut blocks = Vec::with_capacity(number_of_blocks);
                blocks.push(reader.read_blocks()?);
                for _ in 0..number_of_blocks.saturating_sub(1) {
                    let b = debug!(reader.read_blocks()?);
                    blocks.push(b + 1);
                }
                blocks
            };
            debug!(&blocks);

            let res = MaskedIterator::new(
                WebGraphLazyIter::new(reference, ref_node)?, 
                blocks, 
                ref_outdegree
            );
            debug_assert!(nodes_left_to_decode >= res.size);
            debug!(res.size);
            nodes_left_to_decode -= res.size;
            Some(res.peekable())
        };

        // decode intervals
        let intervals = if nodes_left_to_decode == 0 {
            vec![]
        } else {
            let number_of_intervals = reader.read_interval_count()?;
            debug!(number_of_intervals);
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
                debug!(&intervals);
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
                tmp += reader.read_residual()? + 1;
                extra_nodes.push(tmp);
            }
            debug!(&extra_nodes);
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
impl<const QUANTUM_LOG2: usize> WebGraph<RuntimeWebGraphReader<BitArrayM2L<MemoryMappedFileReadOnly>>, QUANTUM_LOG2> {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let properties = Properties::from_file(
            path.with_extension("properties"),
        )?;

        debug!(&properties);

        let nodes_indices = Offsets::from_offsets_file(
            path.with_extension("offsets"),
            properties.clone(),
        )?;

        let mmap = MemoryMappedFileReadOnly::open(
            path.with_extension("graph"),
        )?;

        // create a backend that reads codes from the MSB to the LSb
        let backend_reader =  BitArrayM2L::new(mmap);

        let backend = RuntimeWebGraphReader::new(
            properties.compression_flags.clone(), 
            backend_reader
        );

        Ok(WebGraph{
            properties,
            backend,
            offsets: nodes_indices,
            min_interval_length: 4, // TODO!: Expose
        })
    }
}

impl<Backend: WebGraphReader, const QUANTUM_LOG2: usize> WebGraph<Backend, QUANTUM_LOG2> {
    pub unsafe fn from_raw_parts(backend: Backend, properties: Properties, offsets: Offsets<QUANTUM_LOG2>) -> Self {
        WebGraph {
            properties,
            backend,
            offsets,
            min_interval_length: 4, // TODO!: Expose
        }
    }

    /// Get the degree of a given node
    pub fn get_degree(&self, node_id: usize) -> Result<usize> {
        let mut reader = (&self.backend)
            .get_reader(self.offsets.get(node_id)?);
        reader.read_outdegree()
    }

    /// Get the neighbours of a given node
    pub fn iter_neighbours(&self, node_id: usize) -> Result<WebGraphLazyIter<'_, Backend>> {
        WebGraphLazyIter::new(self, node_id)
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours_and_offset(&self, node_id: usize) -> Result<(usize, Vec<usize>)> {

        // move to the node data
        let index = self.offsets.get(node_id)?;

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

        debug!(&copied_neighbours);
        debug!(&intervals);
        debug!(&extra_nodes);

        let neighbours = three_way_merge(copied_neighbours, intervals, extra_nodes);

        Ok((reader.tell_bits()?, neighbours))
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&self, node_id: usize) -> Result<Vec<usize>> {
        Ok(self.get_neighbours_and_offset(node_id)?.1)
    }

    #[inline]
    fn decode_references(
        &self,
        reader: &mut Backend::WebGraphReaderType<'_>,
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
            copied_neighbours.extend(&self.get_neighbours(ref_node)?);
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
        for node in ref_neighbours {
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
        reader: &mut Backend::WebGraphReaderType<'_>,
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

        let _ = debug!(reader.tell_bits());
        // read the first neighbour
        let first_neighbour_delta = debug!(reader.read_first_residual()?);
        // decode the first neighbour
        let first_neighbour = ((node_id as isize) 
            + debug!(nat2int(first_neighbour_delta))) as usize;
        extra_nodes.push(first_neighbour);

        // decode the other extra nodes
        let mut tmp = first_neighbour;
        for _ in 0..nodes_left_to_decode.saturating_sub(1) {
            let _ = debug!(reader.tell_bits());
            tmp += debug!(debug!(reader.read_residual()?) + 1);
            extra_nodes.push(tmp);
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
