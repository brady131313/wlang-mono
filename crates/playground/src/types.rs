use wasm_bindgen::prelude::*;

use wlang::{
    ast::{walker::TokenContext, NodeKind},
    autocomplete::CompletionTrie,
    lexer::{Token, TokenKind},
    parser::ParseError,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum JSNodeKind {
    Error = "error",
    Workout = "workout",
    Exercise = "exercise",
    SetGroup = "set_group",
    Set = "set",
    Weight = "weight",
    Reps = "reps",
    SimpleDuration = "simple_duration",
    LongDuration = "long_duration",
}

impl From<NodeKind> for JSNodeKind {
    fn from(value: NodeKind) -> Self {
        match value {
            NodeKind::Error => Self::Error,
            NodeKind::Workout => Self::Workout,
            NodeKind::Exercise => Self::Exercise,
            NodeKind::SetGroup => Self::SetGroup,
            NodeKind::Set => Self::Set,
            NodeKind::Weight => Self::Weight,
            NodeKind::Reps => Self::Reps,
            NodeKind::SimpleDuration => Self::SimpleDuration,
            NodeKind::LongDuration => Self::LongDuration,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum JSTokenKind {
    Bodyweight = "bodyweight",
    X = "x",
    Plus = "plus",
    Integer = "integer",
    Float = "float",
    Hour = "hour",
    Minute = "minute",
    Second = "second",
    Colon = "colon",
    Hash = "hash",
    Comma = "comma",
    Newline = "newline",
    Space = "space",
    Ident = "ident",
    Eof = "eof",
    Error = "error",
}

impl From<TokenKind> for JSTokenKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Bodyweight => Self::Bodyweight,
            TokenKind::X => Self::X,
            TokenKind::Plus => Self::Plus,
            TokenKind::Integer => Self::Integer,
            TokenKind::Float => Self::Float,
            TokenKind::Hour => Self::Hour,
            TokenKind::Minute => Self::Minute,
            TokenKind::Second => Self::Second,
            TokenKind::Colon => Self::Colon,
            TokenKind::Hash => Self::Hash,
            TokenKind::Comma => Self::Comma,
            TokenKind::Newline => Self::Newline,
            TokenKind::Space => Self::Space,
            TokenKind::Ident => Self::Ident,
            TokenKind::Eof => Self::Eof,
            TokenKind::Error => Self::Error,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct JSToken {
    kind: JSTokenKind,
    start: u32,
    end: u32,
}

#[wasm_bindgen]
impl JSToken {
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JSTokenKind {
        self.kind
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> u32 {
        self.start
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> u32 {
        self.end
    }
}

impl From<Token> for JSToken {
    fn from(value: Token) -> Self {
        Self {
            kind: value.kind.into(),
            start: value.range.start().into(),
            end: value.range.end().into(),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct JSTokenContext {
    tree_kind: Option<JSNodeKind>,
    token: JSToken,
}

#[wasm_bindgen]
impl JSTokenContext {
    #[wasm_bindgen(getter = treeKind)]
    pub fn tree_kind(&self) -> Option<JSNodeKind> {
        self.tree_kind
    }

    #[wasm_bindgen(getter)]
    pub fn token(&self) -> JSToken {
        self.token
    }
}

impl From<TokenContext> for JSTokenContext {
    fn from(value: TokenContext) -> Self {
        Self {
            tree_kind: value.tree_kind.map(JSNodeKind::from),
            token: JSToken::from(value.token),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct JSParseError(ParseError);

impl From<ParseError> for JSParseError {
    fn from(value: ParseError) -> Self {
        Self(value)
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct JSCompletionTrie(CompletionTrie);

#[wasm_bindgen]
impl JSCompletionTrie {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_exercise(&mut self, exercise: String) {
        self.0.insert_exercises([exercise])
    }

    #[wasm_bindgen]
    pub fn complete_exercise(&self, exercise: &str) -> String {
        let completions: Vec<_> = self.0.complete_exercise(exercise).map(|(e, _)| e).collect();
        format!("{completions:?}")
    }
}
