#![no_main]
extern crate libfuzzer_sys;
extern crate linkify;

use std::str;
use linkify::LinkFinder;

#[export_name="rust_fuzzer_test_input"]
pub extern "C" fn go(data: &[u8]) {
    if let Ok(s) = str::from_utf8(data) {
        let finder = LinkFinder::new();
        finder.links(s).count();
    }
}
