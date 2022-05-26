#[macro_use]
extern crate honggfuzz;
use elias_fano_rust::fuzz::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            codes_harness(data);
        });
    }
}
