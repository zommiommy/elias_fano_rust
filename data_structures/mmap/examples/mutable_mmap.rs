
//! In this example we are going to write to a file using a mmap, and then 
//! read it back with a read only mmap.
const PATH: &str = "test.mmap";

use mmap::*;

fn main() {
    // Get the mutable memory map for the given path, with 800 bytes of size
    let mut mmap = MemoryMapped::new(Some(PATH), Some(800)).unwrap();

    // Get a mutable slice on **all** the memory so we can easily work on it
    let slice = mmap.get_slice_mut::<u64>(0, None).unwrap();
    
    // Write some test data to it
    for i in 0..100 {
        slice[i] = i as u64;
    }

    // Drop everything so we can "simulate" reading the file
    // at a different time
    drop(slice);
    drop(mmap);

    // Create a read onyl memory map
    let read_mmap = MemoryMappedReadOnlyFile::new(PATH).unwrap();
    // Get the slice on all the file
    let slice = read_mmap.get_slice::<u64>(0, None).unwrap();

    // Read back the data
    for i in 0..100 {
        assert_eq!(i as u64, slice[i]);
    }

    // Clean the temporary file
    std::fs::remove_file(PATH).expect("Could not delete the test file");
}