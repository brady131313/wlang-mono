mod utils;

use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use wlang::{
    ast::{self, AstTree, SourceTree, TreeKind, TreeWalker, Workout},
    hir,
    lexer::{lex, TokenKind},
    parser::{parse, ParseError},
};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Default)]
pub struct HTMLPrinter(String);

impl HTMLPrinter {
    fn tag_open(&mut self, tag: &str) {
        self.0.push_str("<span class=\"");
        self.0.push_str(tag);
        self.0.push_str("\">");
    }

    fn tag_close(&mut self) {
        self.0.push_str("</span>");
    }

    fn token_tag(kind: TokenKind) -> Option<&'static str> {
        match kind {
            TokenKind::X => Some("x"),
            TokenKind::Ident => Some("ident"),
            TokenKind::Bodyweight => Some("bw"),
            TokenKind::Error => Some("error"),
            _ => None,
        }
    }

    fn tree_tag(kind: TreeKind) -> Option<&'static str> {
        match kind {
            TreeKind::SetGroup => Some("set-group"),
            TreeKind::Error => Some("error"),
            _ => None,
        }
    }
}

impl TreeWalker for HTMLPrinter {
    type Err = ();

    fn token(&mut self, token: &wlang::ast::Token, source: &str) -> Result<(), ()> {
        let text = match token.kind {
            TokenKind::Space => " ",
            TokenKind::Newline => "\n",
            _ => &source[token.span],
        };

        if let Some(tag) = Self::token_tag(token.kind) {
            self.tag_open(tag);
            self.0.push_str(text);
            self.tag_close();
        } else {
            self.0.push_str(text);
        }

        Ok(())
    }

    fn start_tree(&mut self, kind: TreeKind) -> Result<(), ()> {
        if let Some(tag) = Self::tree_tag(kind) {
            self.tag_open(tag);
        }

        Ok(())
    }

    fn end_tree(&mut self, kind: TreeKind) -> Result<(), ()> {
        if Self::tree_tag(kind).is_some() {
            self.tag_close();
        }

        Ok(())
    }
}

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

    #[wasm_bindgen(js_name = formattedString)]
    pub fn formatted_string(&self) -> String {
        let workout = Workout::cast(&self.tree).unwrap();

        let mut formatter = HTMLPrinter::default();
        workout.walk(&mut formatter, &self.source).unwrap();

        formatter.0
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Vec<JsValue> {
        self.errors
            .iter()
            .map(|e| JsValue::from_serde(e).unwrap())
            .collect()
    }
}

#[wasm_bindgen]
pub struct WorkoutHir(hir::Workout);

#[wasm_bindgen]
impl WorkoutHir {
    #[wasm_bindgen(constructor)]
    pub fn new(cst: WorkoutCst) -> Self {
        let workout = Workout::cast(&cst.tree).unwrap();
        let hir = hir::Workout::lower(workout, &cst.source);
        Self(hir)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.0)
    }
}
