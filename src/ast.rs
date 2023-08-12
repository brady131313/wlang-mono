use crate::lexer::TokenKind;
use std::fmt::{Debug, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeKind {
    Error,
    Workout,
    Exercise,
    SetGroup,
    Set,
    Weight,
    Quantity,
}

#[derive(Debug)]
pub struct Token<'i> {
    pub kind: TokenKind,
    pub text: &'i str,
}

pub struct Tree<'i> {
    pub kind: TreeKind,
    pub children: Vec<Child<'i>>,
}

pub enum Child<'i> {
    Token(Token<'i>),
    Tree(Tree<'i>),
}

impl<'i> Tree<'i> {
    fn print<W: Write>(&self, buf: &mut W, level: usize) -> std::fmt::Result {
        let indent = "  ".repeat(level);
        write!(buf, "{indent}{:?}\n", self.kind)?;

        for child in &self.children {
            match child {
                Child::Token(token) => match token.kind {
                    TokenKind::Space => write!(buf, "{indent}  Space({})\n", token.text.len())?,
                    TokenKind::Newline => write!(buf, "{indent}  Nl({})\n", token.text.len())?,
                    _ => write!(buf, "{indent}  '{}'\n", token.text)?,
                },
                Child::Tree(tree) => tree.print(buf, level + 1)?,
            }
        }

        Ok(())
    }
}

impl<'i> Debug for Tree<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f, 0)
    }
}
