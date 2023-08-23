#![no_main]

use libfuzzer_sys::fuzz_target;
use wlang::lexer::lex;

fuzz_target!(|data: &str| {
    let _ = lex(data);
});
