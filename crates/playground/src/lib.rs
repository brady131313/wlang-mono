mod utils;

use types::JSTokenContext;
use wasm_bindgen::prelude::*;
use wlang::{
    ast::{
        self,
        walker::{SyntaxNodeExt, TreeWalker},
        AstNode, NodeKind, SourceTree, SyntaxTree, Workout,
    },
    hir,
    lexer::TokenKind,
    parser::{parse, ParseError},
};

pub mod types;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused)]
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

    fn node_tag(kind: NodeKind) -> Option<&'static str> {
        match kind {
            NodeKind::SetGroup => Some("set-group"),
            NodeKind::Exercise => Some("exercise"),
            NodeKind::Error => Some("error"),
            _ => None,
        }
    }
}

impl TreeWalker for HTMLPrinter {
    type Err = ();

    fn token(&mut self, token: &ast::SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err> {
        let text = match token.kind(tree) {
            TokenKind::Space => " ",
            TokenKind::Newline => "\n",
            _ => token.text(tree),
        };

        if let Some(tag) = Self::token_tag(token.kind(tree)) {
            self.tag_open(tag);
            self.0.push_str(text);
            self.tag_close();
        } else {
            self.0.push_str(text);
        }

        Ok(())
    }

    fn start_tree(&mut self, node: &ast::SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        if let Some(tag) = Self::node_tag(node.kind(tree)) {
            self.tag_open(tag);
        }

        Ok(())
    }

    fn end_tree(&mut self, node: &ast::SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        if Self::node_tag(node.kind(tree)).is_some() {
            self.tag_close();
        }

        Ok(())
    }
}

#[wasm_bindgen]
pub struct WorkoutCst {
    tree: SyntaxTree,
    errors: Vec<ParseError>,
}

#[wasm_bindgen]
impl WorkoutCst {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        let (tree, errors) = parse(input);

        Self { tree, errors }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        let source_tree = SourceTree::new(&self.tree);
        format!("{source_tree:#?}")
    }

    #[wasm_bindgen(js_name = formattedString)]
    pub fn formatted_string(&self) -> String {
        let mut formatter = HTMLPrinter::default();
        self.tree.root().walk(&mut formatter, &self.tree).unwrap();

        formatter.0
    }

    // #[wasm_bindgen(getter)]
    // pub fn errors(&self) -> Vec<JSParseError> {
    //     self.errors
    //         .iter()
    //         .map(|e| serde_wasm_bindgen::to_value(e).unwrap())
    //         .collect()
    // }

    #[wasm_bindgen(js_name = lookupOffset)]
    pub fn lookup_offset(&self, offset: u32) -> Option<JSTokenContext> {
        self.tree
            .root()
            .lookup_offset(offset, &self.tree)
            .map(JSTokenContext::from)
    }
}

#[wasm_bindgen]
pub struct WorkoutHir(hir::Workout);

#[wasm_bindgen]
impl WorkoutHir {
    #[wasm_bindgen(constructor)]
    pub fn new(cst: &WorkoutCst) -> Self {
        let workout = Workout::cast(cst.tree.root(), &cst.tree).unwrap();
        let hir = hir::Workout::lower(workout, &cst.tree);
        Self(hir)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.0)
    }
}

// #[wasm_bindgen(start)]
// pub fn main() {
//     utils::set_panic_hook()
// }
