#[macro_use]
extern crate honggfuzz;
use elias_fano_rust::*;

fn main() {
    loop {
        fuzz!(|data: Vec<bool>| {
            simple_select_harness(data);
        });
    }
}
