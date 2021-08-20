use super::*;
use rayon::prelude::*;
use rayon::iter::plumbing::{
    bridge_unindexed, 
    UnindexedProducer,
    bridge,
    Producer,
};


#[derive(Debug)]
pub struct SimpleSelectDobuleEndedIterator<'a> {
    /// reference to the SimpleSelect which is being iter
    /// this is needed to get the reference to the high-bits
    father: &'a SimpleSelect,

    start_code: u64,
    start_index: usize,
    end_index: usize,
    end_code: u64,
}

impl<'a> SimpleSelect {
    /// Return an iterator over all the indices of the bits set to one
    /// which are inside the provided range.
    pub fn par_iter_in_range(&'a self, range: Range<u64>) -> SimpleSelectParallelIterator<'a> {
        SimpleSelectParallelIterator{
            father: self,
            range: Some(range),
        }
    }
    
    /// return a ParallelIterator over the indices of the bits set to one in the SimpleSelect.
    pub fn par_iter(&'a self) -> SimpleSelectParallelIterator<'a> {
        SimpleSelectParallelIterator{
            father: self,
            range: None,
        }
    }
}


#[derive(Debug)]
pub struct SimpleSelectParallelIterator<'a> {
    /// reference to the SimpleSelect which is being iter
    /// this is needed to get the reference to the high-bits
    father: &'a SimpleSelect,
    /// The optional range this is done to handle in a consistent way
    /// both `par_iter` and `par_iter_in_range`
    range: Option<Range<u64>>,
}

impl<'a> ParallelIterator for SimpleSelectParallelIterator<'a> {
    type Item = u64;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {

        bridge_unindexed(
            match self.range {
                Some(range) => SimpleSelectIterator::new_in_range(self.father, range),
                None => SimpleSelectIterator::new(self.father),
            },
            consumer,
        )
    }

    fn opt_len(&self) -> Option<usize> {
        self.range.as_ref().map(|r| (r.end - r.start) as usize)
    }
}

impl<'a> IndexedParallelIterator for SimpleSelectParallelIterator<'a> {
    fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        // rank?
    }

    fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        
    }
}

impl<'a> UnindexedProducer for SimpleSelectIterator<'a> {
    type Item = u64;

    /// Split the range in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {

        // Check if it's worth to split
        if (self.max_index - self.index) < 3 {
            return (self, None);
        }

        // Find the middle index for where to split the bitvec
        let mid_index = (self.max_index + self.index) / 2;

        // create a new iterator
        let new = SimpleSelectIterator {
            father: self.father,
            current_code: self.father.high_bits[mid_index],
            index: mid_index,
            max_index: self.max_index,
            max: self.max,
        };

        self.max_index = mid_index;

        (
            self,
            Some(new),
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
