#[macro_use]
extern crate honggfuzz;
use elias_fano_rust::*;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            rank_and_select_harness(data);
        });
    }
}
