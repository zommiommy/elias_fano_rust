use super::*;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::*;

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    /// Return iterator for the values in elias fano.
    #[inline]
    pub fn par_iter(&self) -> impl IndexedParallelIterator<Item = usize> + '_ {
        (0..self.current_number_of_elements as usize)
            .into_par_iter()
            .map(move |index| self.unchecked_select(index as usize))
    }

    /// Return a parallel iterator for the values in elias fano.
    #[inline]
    pub fn par_iter_uniques(&self) -> impl ParallelIterator<Item = usize> + '_ {
        (0..self.current_number_of_elements)
            .into_par_iter()
            .filter_map(move |index| {
                if index == 0 {
                    return Some(self.unchecked_select(0));
                }
                let last_value = self.unchecked_select(index - 1);
                let value = self.unchecked_select(index);

                if last_value != value {
                    Some(value)
                } else {
                    None
                }
            })
    }
}
