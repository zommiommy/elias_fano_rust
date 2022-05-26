
#[inline]
pub fn unchecked_swap(vec: &mut Vec<u64>, a: usize, b:usize) {
    unsafe{
        let pa: *mut u64 = vec.get_unchecked_mut(a);
        let pb: *mut u64 = vec.get_unchecked_mut(b);
        std::ptr::swap(pa, pb);
    }
}

/// Reference classic binary heap
pub struct KAryHeap<const N: usize> {
    values: Vec<u64>
}

impl<const N: usize> KAryHeap<N> {
    /// Initialize a new empty heap
    pub fn new() -> KAryHeap<N> {
        KAryHeap{
            values: Vec::new(),
        }
    }

    /// Initialize a new empty heap which is guaranteed to hold at least 
    /// `capacity` elements without triggering a re-allocation.
    pub fn with_capacity(capacity: usize) -> KAryHeap<N> {
        KAryHeap{
            values: Vec::with_capacity(capacity)
        }
    }

    /// Get the index of the father of the given node
    #[inline]
    fn parent(node: usize) -> usize {
        node.saturating_sub(1) / N
    }

    /// Get the index of the first child of the current node
    #[inline]
    fn first_child(node: usize) -> usize {
        (node * N) + 1
    }

    // If the heap is empty or not
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// add a value to the heap
    pub fn push(&mut self, value: u64) {
        // Insert the value and get its index
        let mut idx = self.values.len();
        self.values.push(value);
        
        // bubble up the value until the heap property holds
        loop {
            let parent_idx = KAryHeap::<N>::parent(idx);

            // The heap condition is respected so we can stop.
            // This also handles the case of the node at the root since
            // self.parent(0) == 0 => current_value == parent_value
            if value >= self.values[parent_idx] {
                break
            }

            // swap the parent and the child
            unchecked_swap(&mut self.values, idx, parent_idx);

            // Update the mutables
            idx = parent_idx;
        }
    }

    /// remove and return the smallest value 
    pub fn pop(&mut self) -> Option<u64> {
        // if the queue is empty we can early-stop.
        if self.values.is_empty() {
            return None;
        }

        // swap the minimum with the last value
        // this is done so we can pop from the end of the vector
        // so we are ensured O(1) complexity.
        let number_of_elements = self.values.len() - 1;
        self.values.swap(0,  number_of_elements);

        // remove the minimum from the tree
        let result = self.values.pop();

        if self.values.is_empty() {
            return result;
        }

        // fix the heap by bubbling down the value
        let mut idx = 0;
        let value = self.values[0];
        loop {
            // get the indices of the right and left child
            let mut min_idx= 0;
            let mut min_value = u64::MAX; 
            
            for i in KAryHeap::<N>::first_child(idx)..KAryHeap::<N>::first_child(idx) + N {
                let v = self.values.get(i).map(|x| *x).unwrap_or(u64::MAX);
                if min_value > v {
                    min_value = v;
                    min_idx = i;
                }
            }
            
            // and the heap rule is violated
            if min_value < value {
                // fix it and keep bubbling down
                unchecked_swap(&mut self.values, idx, min_idx);
                idx = min_idx;   
                continue;
            }
            
            // the min heap rule holds for both childs so we can exit.
            break;
        }

        result
    }
}