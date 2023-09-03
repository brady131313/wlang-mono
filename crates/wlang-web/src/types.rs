use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use wlang::{
    ast::{walker::TokenContext, NodeKind},
    autocomplete::CompletionTrie,
    lexer::{Token, TokenKind},
    parser::ParseError,
};

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub enum JSNodeKind {
    Error,
    Workout,
    Exercise,
    SetGroup,
    Set,
    Weight,
    Reps,
    SimpleDuration,
    LongDuration,
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

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "snake_case")]
pub enum JSTokenKind {
    Bodyweight,
    X,
    Plus,
    Integer,
    Float,
    Hour,
    Minute,
    Second,
    Colon,
    Hash,
    Comma,
    Newline,
    Space,
    Ident,
    Eof,
    Error,
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

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JSToken {
    pub kind: JSTokenKind,
    pub start: u32,
    pub end: u32,
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

#[derive(Debug, Clone, Copy, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JSTokenContext {
    pub tree_kind: Option<JSNodeKind>,
    pub token: JSToken,
}

impl From<TokenContext> for JSTokenContext {
    fn from(value: TokenContext) -> Self {
        Self {
            tree_kind: value.tree_kind.map(JSNodeKind::from),
            token: JSToken::from(value.token),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

    #[wasm_bindgen(js_name = addExercise)]
    pub fn add_exercise(&mut self, exercise: &str) {
        self.0.insert_exercises([exercise])
    }

    #[wasm_bindgen(js_name = completeExercise)]
    pub fn complete_exercise(&self, exercise: &str) -> Vec<JsValue> {
        self.0
            .complete_exercise(exercise)
            .map(|(e, _)| JsValue::from_str(&e))
            .collect()
    }
}
