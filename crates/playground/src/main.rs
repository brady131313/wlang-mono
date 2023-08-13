use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wlang::{
    ast::{AstTree, Workout},
    lexer::lex,
    parser::parse,
};

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
    let (tree, errors) = parse(tokens);
    console_log!("{errors:#?}");

    let workout = Workout::cast(&tree).unwrap();
    for sg in workout.set_groups() {
        if let Some(exercise) = sg.exercise() {
            if let Some(exercise) = exercise.exercise() {
                console_log!("{exercise:?}")
            }
        }
    }

    format!("{tree:#?}")
}

fn main() {
    set_panic_hook();

    console_log!("WASM Loaded");
}
