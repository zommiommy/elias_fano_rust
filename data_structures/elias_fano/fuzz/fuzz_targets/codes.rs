#![no_main]
use libfuzzer_sys::fuzz_target;
use elias_fano_rust::fuzz::*;

fuzz_target!(|data: &[u8]| {
    codes_harness(data)
});
