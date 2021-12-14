//! https://www.kernel.org/doc/Documentation/admin-guide/mm/hugetlbpage.rst
//! https://man7.org/linux/man-pages/man2/mmap.2.html
use crate::errors::*;
use crate::traits::*;

use core::ffi::c_void;
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::Storage::FileSystem::*;


/// A read-only memory mapped file, 
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMappedFileReadOnly {
    file_handle: HANDLE,
    mapping_handle: HANDLE,
    addr: *mut c_void,
    slice: &'static [usize],
    len: usize,
}

impl MemorySlice for MemoryMappedFileReadOnly {
    fn as_ptr(&self) -> *const usize {
        self.slice.as_ptr()
    }
}

impl std::ops::Index<usize> for MemoryMappedFileReadOnly {
    type Output = usize;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.slice[index]
    }
}

impl MemoryFootprint for MemoryMappedFileReadOnly {
    fn total_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

impl Drop for MemoryMappedReadOnlyFile {
    fn drop(&mut self) {
        unsafe {
            let res = UnmapViewOfFile(self.addr);
            if res == BOOL(0) {
                panic!(
                    "Cannot unmap view of file.",
                );
            }

            let res = CloseHandle(self.mapping_handle);
            if res == BOOL(0) {
                panic!(
                    "Cannot Close the mapping handle."
                );
            }

            let res = CloseHandle(self.file_handle);
            if res == BOOL(0) {
                panic!(
                    "Cannot Close the mapping handle."
                );
             }
        }
    }
}

impl MemoryMappedFileReadOnly {
    pub fn open(path: &str) -> Result<Self> { unsafe {
        let file_handle = CreateFileW(
            path,
            FILE_GENERIC_READ,
            FILE_SHARE_NONE,  // prevent other processes to modify the file while we are reading it
            std::ptr::null() as _,
            OPEN_EXISTING,
            FILE_FLAG_SEQUENTIAL_SCAN,
            HANDLE(0),
        );
        
        if file_handle == INVALID_HANDLE_VALUE {
            return Err(
                "Error opening file CreateFileW".into()
            );
        }

        let mut len_higher: u32 = 0;
        let len_lower = GetFileSize(
            file_handle, 
            (&mut len_higher) as *mut u32
        );
        let len = ((len_lower as u64) | (len_higher as u64) << 32) as usize;

        let mapping_handle = CreateFileMappingW(
            file_handle,
            std::ptr::null_mut(),
            PAGE_READONLY, // | SEC_LARGE_PAGES, 
            0, 
            0, 
            PWSTR(std::ptr::null_mut()),
        );
        
        if mapping_handle == HANDLE(0) {
            return Err(
                "Error opening file CreateFileMappingW".into()
            );
        }

        let addr = MapViewOfFile(
            mapping_handle,
            FILE_MAP_READ, // | FILE_MAP_LARGE_PAGES
            0,
            0,
            len,
        );
        
        if addr == std::ptr::null_mut() as _ {
            return Err(
                "Error opening file MapViewOfFile".into()
            );
        }
        
        let slice = std::slice::from_raw_parts(
            self.addr as *const usize, 
            self.len / std::mem::size_of::<usize>(),
        );

        Ok(MemoryMappedReadOnlyFile{
            file_handle,
            mapping_handle,
            addr,
            slice,
            len,
        })
    }
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.slice.len()
    }
}
