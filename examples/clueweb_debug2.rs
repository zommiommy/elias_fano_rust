//! Read sequentially clueweb 12 and compare it against the ascii graph for 
//! correctness

use elias_fano_rust::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mmap = MemoryMappedFileReadOnly::open(
        "/bfd/clueweb12.graph",
    ).unwrap();

    // create a backend that reads codes from the MSB to the LSb
    let backend_reader =  BitArrayM2L::new(mmap);

    let mut reader = backend_reader.get_codes_reader(93112939115);
    println!("{:050b}", reader.read_fixed_length(50).unwrap());

    let mut reader = backend_reader.get_codes_reader(93112939115);
    println!("{}", reader.read_zeta_runtime(3).unwrap());
}
