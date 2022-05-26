#![no_std]
#![feature(generic_associated_types)]

#[cfg(alloc)]
mod vec_impl;

mod slice_impl;

use core::ops::{Range, Index, IndexMut};

/// A general trait of that can read a slice of memory
pub trait Allocation<T>:
    Index<usize, Output = T> 
    + Index<Range<usize>, Output=[T]>
    + AsRef<[T]>
{
    /// return the length of the allocation. This value is how many `T`s the 
    /// allocation can fit.
    fn len(&self) -> usize;
    /// Index in the allocation checking for bounds
    fn get(&self, index: usize) -> Option<&T>;
    /// Index in the allocation without checking for bounds
    unsafe fn get_unchecked(&self, index: usize) -> &T;
}

/// A general trait of that owns a slice of memory and can mutate it
pub trait AllocationMut<T>: Allocation<T> 
    + IndexMut<usize> 
    + IndexMut<Range<usize>>
    + AsMut<[T]>
{
    /// Index in the allocation without checking for bounds
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;
    /// Index in the allocation checking for bounds
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
}

/// A general trait of
pub trait AllocationGrowable<T>: AllocationMut<T> + Extend<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;

    /// Return how many `T` the allocation can contain before having to grow 
    /// and or reallocate
    fn capacity(&self) -> usize;

    /// change the capacity to the given one, this does no initializzation
    fn reserve(&mut self, new_capacity: usize);

    /// Change the capacity filling with `fill_value`
    fn grow(&mut self, new_capacity: usize, fill_value: T);

    /// Reduce the capacity to the minimum needed to store all the data
    fn shrink_to_fit(&mut self);
}