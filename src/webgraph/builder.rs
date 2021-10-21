use super::*;

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

    /// Add a new edge.
    /// This should be called with sorted edges
    //pub fn push(&mut self, src: u64, dst: u64) -> Result<(), String> {
    //
    //}

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
        self.nodes_index.push(self.data.tell() as _);
        // Encode the degree
        self.data.write_gamma(self.current_dsts.len() as _);
        // Write the gaps between the dsts using zeta codes

        // TODO: find which node to ref
        let node_ref = 0;

        // write the reference 
        self.data.write_gamma(self.current_src - node_ref);

        // TODO:  copy list
         

        // Extra nodes
        let first_dst = self.current_dsts[0];
        self.data.write_zeta::<3>(
            if first_dst >= src {
                2 * (first_dst - src)
            } else {
                2 * (src - first_dst) - 1
            }
        );

        let mut tmp = first_dst;
        // TODO!: drain
        for dst in self.current_dsts.iter() {
            self.data.write_zeta::<3>(dst - tmp);
            tmp = *dst;
        }
        self.current_dsts.clear();

        self.current_src = src;
        self.current_dsts.push(dst);
    }

    pub fn get_degree(&mut self, node_id: u64) -> u64 {
        let old_pos = self.data.tell();
        self.data.seek(self.nodes_index[node_id as usize] as usize);
        let res = self.data.read_gamma();
        self.data.seek(old_pos);
        res
    }

    pub fn get_neighbours(&mut self, node_id: u64) -> Vec<u64> {
        let mut neigbours = Vec::new();

        // backup the position so we can reset it later
        let old_pos = self.data.tell();

        // move to the node data
        self.data.seek(self.nodes_index[node_id as usize] as usize);
        
        
        // read the degree
        let degree = self.data.read_gamma();
        // if the degree is 0 we are done and don't need to decode anything
        if degree == 0 {
            self.data.seek(old_pos);
            return vec![];
        }
    
        // figure out the ref
        let ref_delta = self.data.read_gamma();
        if ref_delta != 0 {
            // compute which node we are refering to
            let ref_node = node_id - ref_delta;
            let ref_degree = self.get_degree(node_id);
            // recursive call to decode its neighbours
            let ref_neighbours = self.get_neighbours(ref_node);

            debug_assert_eq!(ref_neighbours.len() as u64, ref_degree);

            // add the nodes to be copied
            for node in ref_neighbours {
                if self.data.read_bit() {
                    neigbours.push(node);
                }
            }
        }

        // read the first neighbour
        let first_neighbour_delta = self.data.read_gamma();
        let first_neighbour = if first_neighbour_delta & 1 == 0 {
            node_id + (first_neighbour_delta >> 1)
        } else {
            node_id - (first_neighbour_delta >> 1)
        };
        neigbours.push(
            first_neighbour
        );

        // decode the other extra nodes
        let mut tmp = first_neighbour;
        for _  in 0..degree - 1 {
            let new_node = self.data.read_zeta::<3>() + tmp;
            tmp = new_node;
        }

        // reset the reader to where it was
        self.data.seek(old_pos);
        neigbours
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