#[macro_use]
extern crate honggfuzz;
use elias_fano_rust::fuzz::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            iter_in_range_harness(data);
        });
    }
}
