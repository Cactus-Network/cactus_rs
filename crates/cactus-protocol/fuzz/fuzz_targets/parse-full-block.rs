#![no_main]
use cactus_protocol::FullBlock;
use cactus_traits::Streamable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = FullBlock::from_bytes(data);
});
