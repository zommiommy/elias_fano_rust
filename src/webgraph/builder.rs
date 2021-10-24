use super::*;

/// FLAG if the struct should use differential compression or not.
const USE_REFERENCES: bool = true;
/// Maximum depth of references, lower values will result in faster
/// decompression but worse compression
const MAX_REFERENCE_RECURSION: u64 = 10;
/// Maximum distance between the current node and the reference one
/// an higher value can result in better compression but slows down 
/// the reference finding algorithm, so the compression will be slower
const MAX_REFERENCE_DISTANCE: u64 = 32;
/// Minimum score that a neighbour has to have to be a reference
const MIN_SCORE_THRESHOLD: usize = 1;
/// If we should use a bitmap for the copy list or
/// the runlength encoding of it
const USE_COPY_LIST: bool = true;
/// If the extra nodes shold be encoded as dense ranges and residuals
/// or just deltas
const USE_INTERVALIZZATION: bool = false;

pub struct WebGraphBuilder {
    current_src: u64,
    current_dsts: Vec<u64>,
    data: BitStream,
    nodes_index: Vec<u64>,
}

impl WebGraphBuilder {
    pub fn new() -> WebGraphBuilder {
        WebGraphBuilder {
            current_src: 0,
            current_dsts: Vec::new(),
            data: BitStream::new(),
            nodes_index: Vec::new(),
        }
    }

    #[inline]
    /// Add a new edge.
    /// This ASSUMES that the edges are sorted and 
    /// violating this assumptions might lead to 
    /// undefined behaviour
    pub unsafe fn push_unchecked(&mut self, src: u64, dst: u64) {
        // we need to encode the degree, so we need to
        // store in a vec the dsts of the current src
        if src == self.current_src {
            self.current_dsts.push(dst);
            return;
        }

        // Save in the outbounds vector the current position
        let index = self.data.tell() as _;
        for _ in self.current_src..src {
            self.nodes_index.push(index);
        }

        // Encode the degree and the neighbours, 
        // we are guaranteed that degree >= 1
        let degree = self.current_dsts.len();
        println!("Encoding: {}, deg: {}, neighbours: {:?}", self.current_src, degree, self.current_dsts);
        
        self.data.write_gamma(degree as _);
        if USE_REFERENCES {
            self.encode_references();
        }
        self.encode_extra_nodes();

        let d = self.get_neighbours(self.current_src);
        assert_eq!(self.current_dsts, d);
        // clean up the 
        self.current_dsts.clear();
        self.current_src = src;
        self.current_dsts.push(dst);
    }

    /// Get the degree of a given node
    pub fn get_degree(&mut self, node_id: u64) -> u64 {
        let old_pos = self.data.tell();
        self.data.seek(self.nodes_index[node_id as usize] as usize);
        let res = self.data.read_gamma();
        self.data.seek(old_pos);
        res
    }

    /// Get the neighbours of a given node
    pub fn get_neighbours(&mut self, node_id: u64) -> Vec<u64> {
        let mut neighbours = Vec::new();

        // backup the position so we can reset it later
        let old_pos = self.data.tell();

        // move to the node data
        let index = self.nodes_index[node_id as usize];
        // if the index is the same as the next one, then
        // we have no stuff to read
        if self.nodes_index.get(1 + node_id as usize)
            .map(|x| index == *x)
            .unwrap_or(false) {
            return vec![];
        }
        self.data.seek(index as usize);
        
        // read the degree
        let degree = self.data.read_gamma();

        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            self.data.seek(old_pos);
            return vec![];
        }
    
        // actually decode the neighbours
        if USE_REFERENCES {
            self.decode_references(node_id, &mut neighbours);
        }
        self.dencode_extra_nodes(node_id, degree, &mut neighbours);

        // reset the reader to where it was
        self.data.seek(old_pos);

        //println!("Decoding: {}, deg: {} neighbours: {:?}", node_id, degree, neighbours);
        neighbours
    }

    #[inline]
    /// Find the best reference for the 
    fn reference_finder(&mut self) -> (u64, Vec<u64>) {        
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

        let mut max_score = 0;
        let mut max_node_id = self.current_src;
        let mut max_neighbours = vec![];

        let min_node_id = self.current_src.saturating_sub(MAX_REFERENCE_DISTANCE);
        for node_id in min_node_id..self.current_src {
            let neighbours = self.get_neighbours(node_id);
            let score = self.current_dsts.iter()
                .filter(|n| neighbours.contains(n))
                .count();
            if score > max_score {
                max_score = score;
                max_node_id = node_id;
                max_neighbours = neighbours;
            }

        }

        // if the reference is sufficently useful, use it
        if max_score >= MIN_SCORE_THRESHOLD {
            (max_node_id, max_neighbours)
        } else {
            (self.current_src, vec![])
        }
    }

    /// Add a new edge.
    /// This should be called with sorted edges
    //pub fn push(&mut self, src: u64, dst: u64) -> Result<(), String> {
    //
    //}

    #[inline]
    fn encode_references(&mut self) {
        let (ref_node_id, ref_neighbours) = self.reference_finder();

        println!("ref_node_id: {}", ref_node_id);
        // write the reference 
        let delta = self.current_src - ref_node_id;
        self.data.write_gamma(delta);

        // If the delta is 0, we don't need to encode the copy list/block 
        if delta == 0 {
            return;
        }

        if USE_COPY_LIST {
            // write the copy list (if any)
            for node in ref_neighbours {
                // TODO! optimize, shit this is slow
                match self.current_dsts.binary_search(&node) {
                    Ok(idx) => {
                        self.data.write_bit(true);
                        self.current_dsts.remove(idx);
                    },
                    Err(_) => {
                        self.data.write_bit(false);
                    }
                }
            }
        } else {
            // Copy Blocks with run length encoding
            unimplemented!("TODO!");
        }
    }

    #[inline]
    fn decode_references(&mut self, node_id: u64, neighbours: &mut Vec<u64>) {
        // figure out the ref
        let ref_delta = self.data.read_gamma();
        if ref_delta != 0 {
            // compute which node we are refering to
            let ref_node = node_id - ref_delta;
            // recursive call to decode its neighbours
            let ref_neighbours = self.get_neighbours(ref_node);

            if USE_COPY_LIST {
                // add the nodes to be copied
                for node in ref_neighbours {
                    if self.data.read_bit() {
                        neighbours.push(node);
                    }
                }
            } else {
                // Copy blocks, decode the run length encoding, and then 
                // proceed as the copy list read
                unimplemented!("TODO!");
            }
        }
    }

    #[inline]
    /// Encode the list of extra nodes as deltas using zeta3 codes.
    fn encode_extra_nodes(&mut self) {
        if USE_INTERVALIZZATION {
            unimplemented!("TODO")
        } else {
            // if there are no extra nodes ignore
            if self.current_dsts.is_empty() {
                return;
            }
            // encode the first extra node
            let first_dst = self.current_dsts[0];
            self.data.write_gamma(
                if first_dst >= self.current_src {
                    2 * (first_dst - self.current_src)
                } else {
                    2 * (self.current_src - first_dst) - 1
                }
            );

            // encode the remaining nodes as deltas
            let mut tmp = first_dst;
            // TODO!: drain
            for dst in self.current_dsts.iter().skip(1) {
                self.data.write_zeta::<3>(dst - tmp);
                tmp = *dst;
            }
        }
    }

    #[inline]
    /// Dencode the list of extra nodes as deltas using zeta3 codes.
    fn dencode_extra_nodes(&mut self, node_id: u64, degree: u64, neighbours: &mut Vec<u64>) {
        if USE_INTERVALIZZATION {
            unimplemented!("TODO")
        } else {
            let nodes_to_decode = degree - neighbours.len() as u64;
            // early stop
            if nodes_to_decode == 0 {
                return;
            }
            // read the first neighbour
            let first_neighbour_delta = self.data.read_gamma();
            // decode the first neighbour
            let first_neighbour = if first_neighbour_delta & 1 == 0 {
                node_id + (first_neighbour_delta >> 1)
            } else {
                node_id - (first_neighbour_delta >> 1) - 1
            };
            neighbours.push(
                first_neighbour
            );

            // decode the other extra nodes
            let mut tmp = first_neighbour;
            for _  in 0..nodes_to_decode - 1 {
                let code = self.data.read_zeta::<3>();
                let new_node = code + tmp;
                neighbours.push(new_node);
                tmp = new_node;
            }
        }
    }


    pub fn build(mut self) -> WebGraph {
        // add a fake node that will not be written to the
        // stream, but forces the flush of the current data
        unsafe{self.push_unchecked(u64::MAX, u64::MAX)};

        // compact the nodes_index array
        let number_of_bits = crate::utils::fast_log2_ceil(self.data.tell() as _);
        let mut compacted_nodes_index = CompactArray::with_capacity(
            number_of_bits as _,
            self.nodes_index.len() as _, 
        );
        for (i, v) in self.nodes_index.iter().enumerate() {
            compacted_nodes_index.write(i as _, *v);
        }

        // return the now-ready 
        WebGraph {
            data: self.data,
            nodes_index: compacted_nodes_index,
        }
    }
}