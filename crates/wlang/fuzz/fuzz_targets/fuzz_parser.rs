#![no_main]

use libfuzzer_sys::fuzz_target;
use wlang::{lexer::lex, parser::parse};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let tokens = lex(s);
        let (_tree, _errors) = parse(tokens);
    }
});
