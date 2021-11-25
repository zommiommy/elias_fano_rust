use core::mem::size_of;

/// Return how much memory the current struct uses
pub trait MemoryFootprint {
    /// Returns the number of bytes it uses
    fn total_size(&self) -> usize;
}

/// Specializzation of `MemoryFootprintConst` for datatypes which memory
/// usage is constantly equal to its size
pub trait MemoryFootprintConst
where
    Self: Sized,
{
    /// Returns the number of bytes it uses
    fn total_size_const() -> usize {
        size_of::<Self>()
    }
}

macro_rules! impl_memory_footprint_for_primitives {
    ($($type:ty)*) => {
        $(
            impl MemoryFootprintConst for $type {}
        )*
    };
}

impl_memory_footprint_for_primitives! {usize isize u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char bool}

impl<T: MemoryFootprintConst> MemoryFootprint for alloc::vec::Vec<T> {
    fn total_size(&self) -> usize {
        // the allocaiton size
        // plus the metadata (ptr to the heap alloc, len, capacity)
        self.len() * T::total_size_const() + (size_of::<usize>() * 3)
    }
}

// impl<T: MemoryFootprint> MemoryFootprint for alloc::vec::Vec<T> {
//     fn total_size(&self) -> usize {
//         // Size of the allocated but not used space
//         (self.capacity() - self.len()) * T::total_size_const()
//         // size of the allocated and used space
//         + self.iter().map(|x| x.total_size()).sum()
//         // plus the metadata (ptr to the heap alloc, len, capacity)
//         + size_of::<usize>() * 3
//     }
// }

impl<T: MemoryFootprintConst, const N: usize> MemoryFootprint for [T; N] {
    fn total_size(&self) -> usize {
        N * T::total_size_const()
    }
}

// impl<T: MemoryFootprint, const N:usize> MemoryFootprint for [T; N] {
//     fn total_size(&self) -> usize {
//         self.iter().map(|x| x.total_size()).sum()
//     }
// }
