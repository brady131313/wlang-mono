#![no_main]

use libfuzzer_sys::fuzz_target;
use wlang::{lexer::lex, parser::parse};

fuzz_target!(|data: &str| {
    let tokens = lex(data);
    let (_tree, _errors) = parse(tokens);
});
