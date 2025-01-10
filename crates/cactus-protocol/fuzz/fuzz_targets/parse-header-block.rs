#![no_main]
use cactus_protocol::HeaderBlock;
use cactus_traits::Streamable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = HeaderBlock::from_bytes(data);
});
