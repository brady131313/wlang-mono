mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello world");
}

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}
