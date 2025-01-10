#![no_main]
use cactus_protocol::Program;
use cactus_traits::Streamable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Program::from_bytes(data);
});
