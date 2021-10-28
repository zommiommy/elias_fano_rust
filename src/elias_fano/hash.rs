use super::*;
use core::hash::{Hash, Hasher};

impl<const QUANTUM_LOG2: usize> Hash for EliasFano<QUANTUM_LOG2> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|x| x.hash(state));
    }
}