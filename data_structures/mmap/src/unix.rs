use libc::*;
use crate::*;

/// A read-only memory mapped file,
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMappedReadOnlyFile {
    fd: i32,
    addr: *mut c_void,
    len: usize,
}

impl std::ops::Drop for MemoryMappedReadOnlyFile {
    fn drop(&mut self) {
        unsafe {
            // unmap the memory
            munmap(self.addr, self.len);
            // close the file descriptor
            close(self.fd);
        }
    }
}

impl MemoryMappedReadOnlyFile {
    pub fn new(path: &str) -> Result<Self> {
        // here we add a + 8 to map in an extra zero-filled word so that we can
        // do unaligned reads for bits
        let len = std::fs::metadata(path)
            .map_err(|_| Error::CannotOpenFile{
                path:path.to_string(),
            })?.len() as usize;

        let mut c_string = path.to_string();
        c_string.push('\0');
        // Get a file descriptor to the file
        let fd = unsafe { open(c_string.as_ptr() as *const i8, O_RDONLY) };

        // check that it was successful
        if fd == -1 {
            return Err(Error::CannotOpenFile{
                path:path.to_string(),
            });
        }
        // Try to mmap the file into memory

        let addr = unsafe {
            mmap(
                // we don't want a specific address
                core::ptr::null_mut(),
                // the len of the file in bytes
                len,
                // Read only
                PROT_READ,
                // We don't want the eventual modifications to get propagated
                // to the underlying file
                libc::MAP_PRIVATE,
                // the file descriptor of the file to mmap
                fd,
                // the offset in bytes from the start of the file, we want to mmap
                // the whole file
                0,
            )
        };

        if addr == usize::MAX as *mut c_void {
            return Err(Error::MMapError);
        }

        Ok(MemoryMappedReadOnlyFile { fd, addr, len })
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get<T>(&self, offset: usize) -> Result<&T> {
        if offset + std::mem::size_of::<T>() > self.len {
            return Err(Error::OutOfBoundReadOnlyAccess{
                idx: offset,
                len: self.len,
            });
        }

        unsafe {
            // get a ptr to the start of the requested slice taking in 
            // consideration offset
            let ptr = (self.addr as *const u8).add(offset);

            // Create the actual slice
            Ok(
                &*(ptr as *const T)
            )
        }
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_unchecked<T>(&self, offset: usize) -> &T {
        // get a ptr to the start of the requested slice taking in 
        // consideration offset
        let ptr = (self.addr as *const u8).add(offset);

        // Create the actual slice
        &*(ptr as *const T)
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub fn get_slice<T>(&self, offset: usize, elements_len: Option<usize>) -> Result<&[T]> {
        let elements_len = elements_len.unwrap_or(
            self.len().saturating_sub(offset) / std::mem::size_of::<T>()
        );
        // Convert from number of elements to number of bytes
        let bytes_len = elements_len * std::mem::size_of::<T>();

        if (offset > self.len) || (bytes_len > self.len - offset) {
            return Err(Error::SliceOutOfBoundReadOnly{
                offset,
                slice_len: bytes_len,
                mmap_len: self.len,
            });
        }

        unsafe {
            // get a ptr to the start of the requested slice taking in 
            // consideration offset
            let ptr = (self.addr as *const u8).add(offset);

            // Create the actual slice
            Ok(
                std::slice::from_raw_parts(
                    ptr as *const T, 
                    elements_len
                )
            )
        }
    }

    /// Returns a new slice of `len` object of type `T` starting from `offset`
    /// bytes from the start of the memory.
    pub unsafe fn get_slice_unchecked<T>(&self, offset: usize, elements_len: Option<usize>) -> &[T] {
        let elements_len = elements_len.unwrap_or(
            self.len().saturating_sub(offset) / std::mem::size_of::<T>()
        );
        // get a ptr to the start of the requested slice taking in 
        // consideration offset
        let ptr = (self.addr as *const u8).add(offset);

        // Create the actual slice
            std::slice::from_raw_parts(
                ptr as *const T, 
                elements_len
            )        
    }

    /// Returns a new str of `len` bytes starting from `offset`
    /// bytes from the start of the memory. 
    /// 
    /// # Safety
    /// This assumes that the data is valid utf8 chars.
    pub fn as_str(&self, offset: usize, len: Option<usize>) -> Result<&str> {
        unsafe{
            Ok(
                std::str::from_utf8_unchecked(
                    self.get_slice::<u8>(offset, len)?
                )
            )
        }
    }
}
