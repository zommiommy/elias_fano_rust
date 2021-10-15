#![no_main]
use libfuzzer_sys::fuzz_target;
use elias_fano_rust::fuzz::*;

fuzz_target!(|data: Vec<bool>| {
    simple_select_harness(data);
});
