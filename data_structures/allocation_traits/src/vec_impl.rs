use crate::*;

impl<T> Allocation<T> for Vec<T>
where
    T: Sync + Send,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }

    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.as_slice().get_unchecked(index)
    }
}

impl<T> AllocationMut<T> for Vec<T>
where
    T: Sync + Send + Clone,
{
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.as_mut_slice().get_unchecked_mut(index)
    }
}

impl<T> AllocationGrowable<T> for Vec<T>
where
    T: Sync + Send + Clone + Copy,
{
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn reserve(&mut self, new_capacity: usize) {
        self.reserve(new_capacity)
    }

    fn grow(&mut self, new_capacity: usize, fill_value: T) {
        self.resize(new_capacity, fill_value)
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }
}