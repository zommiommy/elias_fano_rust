

pub trait GraphUtils {
    // In webgraph we just need to implement these two methods
    fn degree(&self, node_id: u64) -> u64;
    //fn neighbours<F>(&self, src: u64, callback: F) -> impl Iterator<Item=u64>;


    // Returns if an edge is in the current graph
    //fn has_edge(&self, src: u64, dst: u64) -> bool {
    //    for x in self.iter_neighbours(src) {
    //        use std::cmp::Ordering::*;
    //        match x.cmp(dst) {
    //            Greater => return false,
    //            Equal => return true,
    //            Less => {},
    //        }
    //    }
    //}


}