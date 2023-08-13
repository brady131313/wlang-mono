use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wlang::{lexer::lex, parser::parse};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(js_name = parseTree)]
pub fn parse_tree(input: &str) -> String {
    let tokens = lex(input);
    let tree = parse(tokens);
    format!("{tree:#?}")
}

fn main() {
    set_panic_hook();

    console_log!("WASM Loaded");
}
