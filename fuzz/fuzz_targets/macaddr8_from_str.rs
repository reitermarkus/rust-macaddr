#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate macaddr;

use std::str::FromStr;

use macaddr::MacAddr8;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = MacAddr8::from_str(s);
    }
});
