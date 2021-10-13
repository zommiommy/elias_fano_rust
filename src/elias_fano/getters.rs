use super::*;

impl<const QUANTUM_LOG2: usize> EliasFano<QUANTUM_LOG2> {
    #[inline]
    pub fn len(&self) -> usize {
        self.current_number_of_elements as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.current_number_of_elements == 0
    }

    /// Return universe of the elias fano data structure.
    #[inline]
    pub fn get_universe(&self) -> u64 {
        self.universe
    }
}
