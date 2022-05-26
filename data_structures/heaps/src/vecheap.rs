use std::simd::u64x8;

/// 8 Binary heaps that are updated in parallel exploiting SIMD instructions
pub struct VectorizedBinaryHeap {
    /// As a binary heap also this will be implemented in a vector, to exploit
    /// SIMD instructions this now is an Interleaved version of all the heaps
    values: Vec<u64x8>,
}

impl VectorizedBinaryHeap {
    /// Initialize a new empty heap
    pub fn new() -> Self {
        Self{
            values: Vec::new(),
        }
    }

    /// Initialize a new empty heap which is guaranteed to hold at least 
    /// `capacity` elements without triggering a re-allocation.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity((capacity + u64x8::LANES - 1) / u64x8::LANES),
        }
    }

    /// Get the index of the father of the given node
    #[inline]
    fn parent(node: usize) -> usize {
        (node.saturating_sub(1)) >> 1
    }

    /// Get the index of the left child
    #[inline]
    fn left(node: usize) -> usize {
        (node << 1) + 1
    }

    /// Get the index of the right child
    #[inline]
    fn right(node: usize) -> usize {
        (node << 1) + 2
    }

    // If the heap is empty or not
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// add a 8 values to the heaps
    pub fn push(&mut self, value: u64x8) {
        // Insert the value and get its index
        let mut idx = self.values.len();
        self.values.push(value);
        
        // bubble up the value until the heap property holds
        loop {
            let parent_idx = Self::parent(idx);
            // The heap condition is respected so we can stop.
            // This also handles the case of the node at the root since
            // self.parent(0) == 0 => current_value == parent_value
            let mask = value.lanes_ge(self.values[parent_idx]);
            if mask.all() {
                break
            }

            // swap the parent and the child
            let child_values  = mask.select(value, self.values[parent_idx]);
            let parent_values = (!mask).select(value, self.values[parent_idx]);
            
            debug_assert!(child_values.lanes_ge(parent_values).all());

            self.values[idx] = child_values;
            self.values[parent_idx] = parent_values;

            // Update the mutables
            idx = parent_idx;
        }
    }
}
