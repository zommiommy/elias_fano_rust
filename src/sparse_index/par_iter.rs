use super::*;
use crate::constants::*;
use rayon::iter::plumbing::UnindexedProducer;

/// This isn't tested, as for elias-fano we need the indexed version
/// and for a general parllalel iterator we can use the normal iter
/// which is slightly faster.
///
/// Thus, this trait is not really needed, but we have it ¯\_(ツ)_/¯ .
impl<'a, const QUANTUM_LOG2: usize> UnindexedProducer
    for SparseIndexDobuleEndedIterator<'a, QUANTUM_LOG2>
{
    type Item = usize;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split
        if self.len() < 2 {
            return (self, None);
        }

        // compute the current parsing index
        let start_value =
            (self.start_index as usize * WORD_BIT_SIZE) + self.start_code.trailing_zeros() as usize;

        // compute how many ones there where
        let start_rank = self.father.rank1(start_value) as usize;
        // Compute the middle 1 in the current iterator
        let middle_point = start_rank + (self.len / 2);
        // Find it's index, so we can split the iterator exactly in half
        let middle_bit_index = self.father.select1(middle_point as usize);
        let code = self.father.high_bits[middle_bit_index as usize];
        let inword_offset = middle_bit_index & WORD_BIT_SIZE_MASK;

        // Create the new iterator for the second half
        let new_iter = SparseIndexDobuleEndedIterator {
            father: self.father,

            start_code: code & !(usize::MAX << inword_offset),
            start_index: (middle_bit_index >> WORD_SHIFT) as usize,

            end_index: self.end_index,
            end_code: self.end_code,

            len: self.len() - middle_point,
        };

        // Update the current iterator so that it will work on the
        // first half
        self.end_index = (middle_bit_index >> WORD_SHIFT) as usize;
        self.end_code = code & (usize::MAX << inword_offset);
        self.len = middle_point;

        // return the two halfs
        (self, Some(new_iter))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}

// impl<'a> Producer for SparseIndexDobuleEndedIterator<'a> {
//     fn into_iter(self) -> Self::IntoIter {
//         self
//     }
//
//     fn split_at(self, index: usize) -> (Self, Self) {
//
//     }
// }
