//! https://www.kernel.org/doc/Documentation/admin-guide/mm/hugetlbpage.rst
//! https://man7.org/linux/man-pages/man2/mmap.2.html
use crate::errors::*;
use crate::traits::*;
use libc::*;

/// A read-only memory mapped file, 
/// this should be equivalent to read-only slice that
/// automatically handle the freeing.
#[derive(Debug)]
pub struct MemoryMappedFileReadOnly {
    fd: i32,
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

impl std::ops::Drop for MemoryMappedFileReadOnly {
    fn drop(&mut self) {
        unsafe{
            // unmap the memory
            munmap(self.addr, self.len);
            // close the file descriptor
            close(self.fd);
        }
    }
}

impl MemoryMappedFileReadOnly {
    pub fn open(path: &str, flags: i32) -> Result<Self> {
        // here we add a + 8 to map in an extra zero-filled word so that we can
        // do unaligned reads for bits
        let mut len = 1 + std::fs::metadata(path)
            .map_err(|e| Error::OpenError(e))?
            .len() as usize;
        // padd the vector to be a multiple of 8 bytes
        len += 8 - (len % 8);

        let mut c_string = path.to_string();
        c_string.push('\0');
        // Get a file descriptor to the file
        let fd = unsafe{open(
            c_string.as_ptr() as *const i8,
            O_RDONLY
        )};

        // check that it was successful
        if fd == -1 {
            return Err(
                Error::UnableToOpenFile(
                    format!("Cannot open the file '{}' to mmap it.", 
                    path)
                )
            );
        }

        // Try to mmap the file into memory
        let addr = unsafe{mmap(
            // we don't want a specific address
            core::ptr::null_mut(),
            // the len of the file in bytes
            len,
            // Read only
            PROT_READ,
            // We don't want the eventual modifications to get propagated
            // to the underlying file
            flags,
            // the file descriptor of the file to mmap
            fd,
            // the offset in bytes from the start of the file, we want to mmap
            // the whole file
            0,
        )};


        if addr == usize::MAX as *mut c_void {
            return Err(
                Error::MMapError(
                    format!(concat!(
                        "Cannot mmap the file '{}' with file descriptor '{}'.",
                        " Errno: {} for more info see ",
                        "https://man7.org/linux/man-pages/man2/mmap.2.html",
                        ),
                        path, fd, unsafe{*libc::__errno_location()}
                    )
                )
            );
        }

        let slice = unsafe{core::slice::from_raw_parts(
            addr as *const usize, 
            len / core::mem::size_of::<usize>()
        )};

        Ok(MemoryMappedFileReadOnly{
            fd,
            len,
            addr,
            slice,
        })
    }

    /// Return the number of `usize` words in the slice
    pub fn len(&self) -> usize {
        self.slice.len()
    }
}
