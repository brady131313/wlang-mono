use wasm_bindgen::prelude::*;
use wlang::{
    ast::{walker::TokenContext, Token, TreeKind},
    lexer::TokenKind,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum JSTreeKind {
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

impl From<TreeKind> for JSTreeKind {
    fn from(value: TreeKind) -> Self {
        match value {
            TreeKind::Error => Self::Error,
            TreeKind::Workout => Self::Workout,
            TreeKind::Exercise => Self::Exercise,
            TreeKind::SetGroup => Self::SetGroup,
            TreeKind::Set => Self::Set,
            TreeKind::Weight => Self::Weight,
            TreeKind::Reps => Self::Reps,
            TreeKind::SimpleDuration => Self::SimpleDuration,
            TreeKind::LongDuration => Self::LongDuration,
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
pub struct JSToken {
    kind: JSTokenKind,
    start: u32,
    end: u32,
}

impl From<Token> for JSToken {
    fn from(value: Token) -> Self {
        Self {
            kind: value.kind.into(),
            start: value.span.start().into(),
            end: value.span.end().into(),
        }
    }
}

#[wasm_bindgen]
pub struct JSTokenContext {
    tree_kind: Option<JSTreeKind>,
    token: Token,
}

impl From<TokenContext> for JSTokenContext {
    fn from(value: TokenContext) -> Self {
        Self {
            tree_kind: value.tree_kind.map(JSTreeKind::from),
            token: value.token,
        }
    }
}

#[wasm_bindgen]
impl JSTokenContext {
    #[wasm_bindgen(getter = treeKind)]
    pub fn tree_kind(&self) -> Option<JSTreeKind> {
        self.tree_kind
    }

    #[wasm_bindgen(getter)]
    pub fn token(&self) -> String {
        format!("{}", self.token)
    }
}
