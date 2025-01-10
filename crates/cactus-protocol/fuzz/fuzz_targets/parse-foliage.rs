#![no_main]
use cactus_protocol::Foliage;
use cactus_traits::Streamable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Foliage::from_bytes(data);
});
