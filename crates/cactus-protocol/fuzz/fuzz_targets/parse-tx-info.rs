#![no_main]
use cactus_protocol::TransactionsInfo;
use cactus_traits::Streamable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = TransactionsInfo::from_bytes(data);
});
