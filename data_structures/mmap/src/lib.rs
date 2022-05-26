//! Portable OS-agnostic memory mapping of files.
//! Currently we support Linux, Windows and MacOs.
//! 
//! https://www.kernel.org/doc/Documentation/admin-guide/mm/hugetlbpage.rst
//! https://man7.org/linux/man-pages/man2/mmap.2.html
//! https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile
//! https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createfilemappinga
//! https://docs.microsoft.com/en-us/windows/win32/memory/creating-a-file-mapping-object

/// Re-export allocation traits because without them the mmaps are not usable
pub use allocation_traits::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::MemoryMappedReadOnlyFile;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::MemoryMappedReadOnlyFile;

//#[cfg(not(target_os = "windows"))]
//mod unix_mut;
//#[cfg(not(target_os = "windows"))]
//pub use unix_mut::MemoryMapped;

pub type Result<T> = core::result::Result<T, crate::Error>;

#[derive(Debug)]
pub enum Error {
    /// This error is returned when the memory map can't open the wanted file
    CannotOpenFile{
        path: String,
    },
    OutOfBoundReadOnlyAccess{
        idx: usize,
        len: usize,
    },
    OutOfBoundMutAccess{
        idx: usize,
        len: usize,
    },
    SliceOutOfBoundReadOnly{
        offset: usize,
        slice_len: usize,
        mmap_len: usize,
    },
    SliceOutOfBoundMut{
        offset: usize,
        slice_len: usize,
        mmap_len: usize,
    },
    MMapError,
}

unsafe impl Sync for MemoryMappedReadOnlyFile {}
unsafe impl Send for MemoryMappedReadOnlyFile {}
