
/// Reference classic binary heap
pub struct PriorityQueue {
    values: Vec<u64>,
    keys  : Vec<u64>,
}

impl PriorityQueue {
    /// Initialize a new empty heap
    pub fn new() -> Self {
        PriorityQueue{
            values: Vec::new(),
            keys  : Vec::new(),
        }
    }

    /// Initialize a new empty heap which is guaranteed to hold at least 
    /// `capacity` elements without triggering a re-allocation.
    pub fn with_capacity(capacity: usize) -> Self {
        PriorityQueue{
            values: Vec::with_capacity(capacity),
            keys  : Vec::with_capacity(capacity),
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

    /// add a value to the heap
    pub fn push(&mut self, key: u64, value: u64) {
        // Insert the value and get its index
        let mut idx = self.values.len();
        self.keys.push(key);
        self.values.push(value);
        
        // bubble up the value until the heap property holds
        loop {
            let parent_idx = PriorityQueue::parent(idx);

            // The heap condition is respected so we can stop.
            // This also handles the case of the node at the root since
            // self.parent(0) == 0 => current_value == parent_value
            if value >= self.values[parent_idx] {
                break
            }

            // swap the parent and the child
            self.values.swap(idx, parent_idx);
            self.keys.swap(idx, parent_idx);

            // Update the mutables
            idx = parent_idx;
        }
    }

    /// remove and return the smallest value 
    pub fn pop(&mut self) -> Option<(u64, u64)> {
        // if the queue is empty we can early-stop.
        if self.values.is_empty() {
            return None;
        }

        // swap the minimum with the last value
        // this is done so we can pop from the end of the vector
        // so we are ensured O(1) complexity.
        let number_of_elements = self.values.len() - 1;
        self.keys.swap(0, number_of_elements);
        self.values.swap(0,  number_of_elements);

        // remove the minimum from the tree
        let result_val = unsafe{self.values.pop().unwrap_unchecked()};
        let result_key = unsafe{self.keys.pop().unwrap_unchecked()};

        if self.values.is_empty() {
            return Some((result_key, result_val));
        }

        // fix the heap by bubbling down the value
        let mut idx = 0;
        let value = self.values[0];
        loop {
            // get the indices of the right and left child
            let left_i = PriorityQueue::left(idx);
            let right_i = PriorityQueue::right(idx);
            let left_v = self.values.get(left_i).map(|x| *x).unwrap_or(u64::MAX);
            let right_v = self.values.get(right_i).map(|x| *x).unwrap_or(u64::MAX);

            // find the smallest child
            let (smallest_i, smallest_v) = if left_v > right_v {
                (right_i, right_v)
            } else {
                (left_i, left_v)
            };

            // and the heap rule is violated
            if smallest_v < value {
                // fix it and keep bubbling down
                self.keys.swap(idx, smallest_i);
                self.values.swap(idx, smallest_i);
                idx = smallest_i;   
                continue;
            }
            
            // the min heap rule holds for both childs so we can exit.
            break;
        }

        Some((result_key, result_val))
    }
}
