#![no_main]

use libfuzzer_sys::fuzz_target;
use wlang::{ast::PlainPrinter, lexer::lex, parser::parse};

fuzz_target!(|data: &str| {
    let tokens = lex(data);
    let (tree, _errors) = parse(tokens);

    let mut printer = PlainPrinter::default();
    tree.walk(&mut printer, data).unwrap();
    assert_eq!(data, printer.take());
});
