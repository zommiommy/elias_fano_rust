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
        const ZETA_K: u64 = 3;
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

        let first_dst = self.current_dsts[0];
        self.data.write_zeta::<ZETA_K>(
            if first_dst >= src {
                2 * (first_dst - src)
            } else {
                2 * (src - first_dst) - 1
            }
        );

        let mut tmp = first_dst;
        // TODO!: drain
        for dst in self.current_dsts.iter() {
            self.data.write_zeta::<ZETA_K>(dst - tmp);
            tmp = *dst;
        }
        self.current_dsts.clear();

        self.current_src = src;
        self.current_dsts.push(dst);
    }


    pub fn build(mut self) -> WebGraph {
        // add a fake node that will not be written to the
        // stream, but forces the flush of the current data
        unsafe{self.push_unchecked(u64::MAX, u64::MAX)};

        // compact the nodes_index array
        let number_of_bits = crate::utils::fast_log2(self.data.tell() as _);
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