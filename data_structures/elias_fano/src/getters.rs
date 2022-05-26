use super::*;

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    #[inline]
    /// Return the current number of elements pushed to Elias-Fano
    pub fn len(&self) -> usize {
        self.current_number_of_elements as usize
    }

    #[inline]
    /// Return if there was at least a push to this structure
    pub fn is_empty(&self) -> bool {
        self.current_number_of_elements == 0
    }

    /// Return universe of the elias fano data structure.
    #[inline]
    pub fn get_universe(&self) -> usize {
        self.universe
    }
}
