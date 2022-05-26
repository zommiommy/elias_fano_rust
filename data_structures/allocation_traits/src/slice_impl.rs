use crate::*;

impl<T: Sync + Send> Allocation<T> for [T] {
    fn len(&self) -> usize {
        <[T]>::len(&self)
    }
    fn get(&self, index: usize) -> Option<&T> {
        <[T]>::get(self, index)
    }
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        <[T]>::get_unchecked(self, index)
    }
}

impl<T: Sync + Send> AllocationMut<T> for [T] {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        <[T]>::get_mut(self, index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        <[T]>::get_unchecked_mut(self, index)
    }
}