use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wlang::{
    ast::{AstTree, TreeKind, TreeWalker, Workout},
    lexer::{lex, TokenKind},
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
    fn token(&mut self, token: &wlang::ast::Token) {
        let text = match token.kind {
            TokenKind::Space => " ",
            TokenKind::Newline => "\n",
            _ => token.text,
        };

        if let Some(tag) = Self::token_tag(token.kind) {
            self.tag_open(tag);
            self.0.push_str(text);
            self.tag_close();
        } else {
            self.0.push_str(text);
        }
    }

    fn start_tree(&mut self, kind: TreeKind) {
        if let Some(tag) = Self::tree_tag(kind) {
            self.tag_open(tag);
        }
    }

    fn end_tree(&mut self, kind: TreeKind) {
        if Self::tree_tag(kind).is_some() {
            self.tag_close();
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct ParseResult {
    pub cst_str: String,
    pub formatted_str: String,
}

#[wasm_bindgen(js_name = parseTree)]
pub fn parse_tree(input: &str) -> ParseResult {
    let tokens = lex(input);
    let (tree, errors) = parse(tokens);
    console_log!("{errors:#?}");

    let workout = Workout::cast(&tree).unwrap();

    let mut formatter = HTMLPrinter::default();
    workout.walk(&mut formatter);

    let cst_str = format!("{tree:#?}");

    ParseResult {
        cst_str,
        formatted_str: formatter.0,
    }
}

fn main() {
    set_panic_hook();

    console_log!("WASM Loaded");
}
