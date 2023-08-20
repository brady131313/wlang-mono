mod utils;

use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use wlang::{
    ast::{self, SourceTree},
    lexer::lex,
    parser::{parse, ParseError},
};

#[wasm_bindgen]
pub struct WorkoutCst {
    tree: ast::Tree,
    source: String,
    errors: Vec<ParseError>,
}

#[wasm_bindgen]
impl WorkoutCst {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        let tokens = lex(input);
        let (tree, errors) = parse(tokens);

        Self {
            tree,
            source: input.to_string(),
            errors,
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        let source_tree = SourceTree::new(&self.source, &self.tree);
        format!("{source_tree:#?}")
    }

    // #[wasm_bindgen(js_name = formattedString)]
    // pub fn formatted_string(&self) -> String {
    //     let workout = Workout::cast(&self.tree).unwrap();
    //
    //     let mut formatter = HTMLPrinter::default();
    //     workout.walk(&mut formatter, &self.source).unwrap();
    //
    //     formatter.0
    // }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<JsValue> {
        self.errors
            .iter()
            .map(|e| JsValue::from_serde(e).unwrap())
            .collect()
    }
}
