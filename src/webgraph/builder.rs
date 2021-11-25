use super::*;
use crate::Result;

use alloc::{vec, vec::Vec};
use core::convert::TryInto;
use core::mem::size_of;
#[cfg(feature = "par_iter")]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
pub const USE_INTERVALIZZATION: bool = false;
pub const MIN_INTERVALIZZATION_LEN: usize = 3;
// If during the compression we should employ the swap_remove trick
// which requires then sorting the current_dsts possibly multiple times
// TODO!: Finish validating this
pub const SWAP_REMOVE: bool = false;

/// Builder for WebGraph, the only difference to webgraph is that this uses a
/// vecotr of usizes to index the nodes, while actual webgraph uses a compressed
/// array.
pub struct WebGraphBuilder<BACKEND: WebGraphBackend> {
    data: WebGraph<BACKEND>,

    neighbours_cache: [(usize, usize, Vec<usize>); MAX_REFERENCE_DISTANCE],
    neighbours_cache_index: usize,
}

impl<BACKEND: WebGraphBackend> crate::traits::MemoryFootprint for WebGraphBuilder<BACKEND> {
    fn total_size(&self) -> usize {
        self.data.total_size() //TODO!: add caches sizes
    }
}

impl<BACKEND: WebGraphBackend> WebGraphBuilder<BACKEND> {
    /// Create a new builder over the given backend
    pub fn new(backend: BACKEND) -> WebGraphBuilder<BACKEND> {
        WebGraphBuilder {
            data: backend,
            nodes_index: Vec::new(),
            neighbours_cache: vec![(0, 0, Vec::new()); MAX_REFERENCE_DISTANCE]
                .try_into()
                .unwrap(),
            neighbours_cache_index: 0,
        }
    }

    #[inline]
    /// Add a new edge.
    /// This ASSUMES that the edges are sorted and
    /// violating this assumptions might lead to
    /// undefined behaviour
    pub unsafe fn push_unchecked(&mut self, src: usize, dsts: &[usize]) -> Result<()> {
        // Save in the outbounds vector the current position
        let index = self.data.tell_bits()? as _;
        for _ in self.nodes_index.len()..=self.current_src as usize {
            self.nodes_index.push(index);
        }

        // backup the dsts to be added to the cache
        let old_dts = self.current_dsts.clone();
        // Encode the degree and the neighbours,
        // we are guaranteed that degree >= 1
        let degree = self.current_dsts.len();
        self.data.write_gamma(degree as _)?;
        if USE_REFERENCES {
            let ref_depth = self.encode_references()?;
            // update the cache
            if ref_depth < MAX_REFERENCE_RECURSION {
                self.neighbours_cache[self.neighbours_cache_index % MAX_REFERENCE_DISTANCE] =
                    (self.current_src, ref_depth, old_dts);
                self.neighbours_cache_index = self.neighbours_cache_index.wrapping_add(1);
            }
        }
        self.encode_extra_nodes()?;

        Ok(())
    }

    #[inline]
    /// Find the best reference for the
    fn reference_finder(&mut self) -> (usize, usize, Vec<usize>) {
        // TODO!: write an actually good algorithm
        // we could compute explicitely how big the encoding
        // would be using each node as a ref, so we have an exact
        // score to minimize

        // Other ideas:
        // Iter over the previous nodes (up to a limit maybe)
        // decode shit and do fast SIMD set intersection
        // since we have sorted arrays
        // We can both limit the distance between nodes
        // and the depth of recursion for speed sake
        // also we can simulate the encoding and figue out
        // if it's good to reference or not using a cost model
        // e.g. if we need to copy just a node it's not worth to
        // bother with references so fuckit just encode the extra
        // node, for memory we can do it exactly (maybe slow)
        // or we can approximate it by setting a threshold of
        // at least 5 edges shared, and distance < 1_000
        // or some shit like that

        #[cfg(feature = "par_iter")]
        let iter = self.neighbours_cache.par_iter();
        #[cfg(not(feature = "par_iter"))]
        let iter = self.neighbours_cache.iter();

        // make an immutable reference to the dsts so that in the parallel case
        // we can just share that (which is safe because it's immutable)
        // so we don't need to require Sync for the Backend.
        let dsts_reference = &self.current_dsts;

        let (max_score, max_node_id, max_depth, max_neighbours) = iter
            .map(|(node_id, depth, neighbours)| {
                // Compute how many neighbours the node share with the
                // current_src assuming that both vecs are sorted!
                let mut sharing_count = 0;
                let mut i = 0;
                let mut j = 0;

                while let (Some(n1), Some(n2)) = (dsts_reference.get(i), neighbours.get(j)) {
                    use core::cmp::Ordering;
                    match n1.cmp(n2) {
                        Ordering::Equal => {
                            i += 1;
                            j += 1;
                            sharing_count += 1;
                        }
                        Ordering::Less => {
                            i += 1;
                        }
                        Ordering::Greater => {
                            j += 1;
                        }
                    }
                }

                (sharing_count as f64, node_id, depth, neighbours)
            })
            .max_by(|(score_a, node_id_a, _, _), (score_b, node_id_b, _, _)| {
                match score_a.partial_cmp(score_b).unwrap() {
                    core::cmp::Ordering::Equal => node_id_a.cmp(node_id_b),
                    x @ _ => x,
                }
            })
            .unwrap();

        // if the reference is sufficently useful, use it
        if max_score >= MIN_SCORE_THRESHOLD {
            // TODO!: can we remove this clone?
            (*max_node_id, *max_depth, max_neighbours.clone())
        } else {
            (self.current_src, 0, vec![])
        }
    }

    #[inline]
    fn encode_references(&mut self) -> Result<usize> {
        let (ref_node_id, ref_depth, ref_neighbours) = self.reference_finder();
        // write the reference
        let delta = self.current_src - ref_node_id;
        self.data.write_gamma(delta)?;

        // If the delta is 0, we don't need to encode the copy list/block
        if delta == 0 {
            return Ok(ref_depth);
        }

        if USE_COPY_LIST {
            // write the copy list (if any)
            for node in ref_neighbours {
                // TODO! optimize, shit this is slow
                match self.current_dsts.binary_search(&node) {
                    Ok(idx) => {
                        self.data.write_bit(true)?;
                        if SWAP_REMOVE {
                            self.current_dsts.swap_remove(idx);
                        } else {
                            self.current_dsts.remove(idx);
                        }
                    }
                    Err(_) => {
                        self.data.write_bit(false)?;
                    }
                }
            }
        } else {
            // Copy Blocks with run length encoding
            let mut current_bit_value = true;
            let mut counter = 0;
            let mut blocks = Vec::new();
            // write the copy list (if any)
            for node in ref_neighbours {
                // TODO! optimize this
                let curr_bit = match self.current_dsts.binary_search(&node) {
                    Ok(idx) => {
                        if SWAP_REMOVE {
                            self.current_dsts.swap_remove(idx);
                        } else {
                            self.current_dsts.remove(idx);
                        }
                        true
                    }
                    Err(_) => false,
                };

                // during run length encoding change the enc at each rising /
                // falling edges
                if curr_bit != current_bit_value {
                    blocks.push(counter);
                    current_bit_value = curr_bit;
                    counter = 0;
                }
                counter += 1;
            }
            if counter > 0 {
                blocks.push(counter);
            }
            self.data.write_gamma(blocks.len() as usize)?;
            // TODO!: should this be a panic?
            self.data.write_gamma(blocks[0])?;
            for counter in &blocks[1..] {
                self.data.write_gamma(*counter - 1)?;
            }
        }
        Ok(ref_depth + 1)
    }
    #[inline]
    /// Encode the list of extra nodes as deltas using zeta3 codes.
    fn encode_extra_nodes(&mut self) -> Result<()> {
        // if there are no extra nodes ignore
        if self.current_dsts.is_empty() {
            return Ok(());
        }

        if SWAP_REMOVE || USE_INTERVALIZZATION {
            self.current_dsts.sort();
        }

        if USE_INTERVALIZZATION {
            // ensure that the dsts are sorted
            // during the ref for speed sake we can brake sorting
            let mut counter = 0;
            let mut start = self.current_dsts[0];
            let mut ranges = Vec::new();
            // compute the ranges of sequential values big enought to be encoded
            for node_id in &self.current_dsts[1..] {
                if *node_id == start + counter {
                    counter += 1;
                } else {
                    if counter > MIN_INTERVALIZZATION_LEN {
                        ranges.push((start, counter));
                    }
                    start = *node_id;
                    counter = 0;
                }
            }

            // encode the ranges and remove the values form the current_dsts
            self.data.write_gamma(ranges.len() as _)?;

            for (start, delta) in ranges {
                // encode the ranges
                self.data.write_gamma(start)?;
                self.data.write_gamma(delta)?;
                // delete the nodes
                for id in start..start + delta {
                    let idx = self.current_dsts.binary_search(&id).unwrap();
                    if SWAP_REMOVE {
                        self.current_dsts.swap_remove(idx);
                    } else {
                        self.current_dsts.remove(idx);
                    }
                }
            }

            if SWAP_REMOVE {
                self.current_dsts.sort();
            }
        }

        // encode the first extra node
        let first_dst = self.current_dsts[0];
        self.data.write_gamma(if first_dst >= self.current_src {
            2 * (first_dst - self.current_src)
        } else {
            2 * (self.current_src - first_dst) - 1
        })?;

        // encode the remaining nodes as deltas
        let mut tmp = first_dst;
        // TODO!: drain
        for dst in self.current_dsts.iter().skip(1) {
            // If we don't have multigraphs we can do - 1 in the delta
            // and save 0.1 bits per edge (Experimental)
            self.data.write_zeta::<3>(dst - tmp)?;
            tmp = *dst;
        }

        Ok(())
    }

    /// Compress the node index and bvuild the last indices
    pub fn build(mut self) -> Result<WebGraph<BACKEND>> {
        // add a fake node that will not be written to the
        // stream, but forces the flush of the current data
        unsafe { self.push_unchecked(usize::MAX, usize::MAX)? };

        // compact the nodes_index array
        // let number_of_bits = crate::utils::fast_log2_ceil(self.data.tell_bits()? as _);
        // let mut compacted_nodes_index = CompactArray::with_capacity(
        //     number_of_bits as _,
        //     self.nodes_index.len() as _,
        // );
        // for (i, v) in self.nodes_index.iter().enumerate() {
        //     compacted_nodes_index.write(i as _, *v);
        // }

        // return the now-ready
        Ok(WebGraph {
            data: self.data,
            nodes_index: self.nodes_index,
        })
    }
}
