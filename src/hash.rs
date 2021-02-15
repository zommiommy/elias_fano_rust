use super::*;
use std::hash::{Hash, Hasher};

impl Hash for EliasFano {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.universe.hash(state);
        self.number_of_elements.hash(state);
        self.low_bits.hash(state);
        self.high_bits.hash(state);
        
        // These are helper methods and should not matter
        // self.last_high_value.hash(state);
        // self.last_value.hash(state);
        // self.last_index.hash(state);
        // self.current_number_of_elements.hash(state);
        // self.low_bit_count.hash(state);
        // self.low_bit_mask.hash(state);
    }
}