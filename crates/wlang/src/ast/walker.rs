use std::fmt::{Display, Write};

use eventree::{TextRange, TextSize};

use crate::lexer::{Token, TokenKind};

use super::{NodeKind, SyntaxNode, SyntaxToken, SyntaxTree};

pub trait TreeWalker {
    type Err;

    fn token(&mut self, token: &SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err>;

    fn start_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err>;

    fn end_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err>;
}

pub trait SyntaxNodeExt {
    fn walk<W: TreeWalker>(&self, walker: &mut W, tree: &SyntaxTree) -> Result<(), W::Err>;

    fn lookup_span(&self, span: TextRange, tree: &SyntaxTree) -> Option<TokenContext> {
        let mut walker = LookupSpan::new(span);

        if let Err(token) = self.walk(&mut walker, tree) {
            return Some(TokenContext {
                token,
                tree_kind: walker.current_tree_kind,
            });
        } else {
            return None;
        }
    }

    fn lookup_offset<O: Into<TextSize>>(
        &self,
        offset: O,
        tree: &SyntaxTree,
    ) -> Option<TokenContext> {
        let span = TextRange::empty(offset.into());
        self.lookup_span(span, tree)
    }
}

impl SyntaxNodeExt for SyntaxNode {
    fn walk<W: TreeWalker>(&self, walker: &mut W, tree: &SyntaxTree) -> Result<(), W::Err> {
        walker.start_tree(&self, tree)?;

        for child in self.children(tree) {
            match child {
                eventree::SyntaxElement::Node(node) => node.walk(walker, tree)?,
                eventree::SyntaxElement::Token(token) => walker.token(&token, tree)?,
            }
        }

        walker.end_tree(&self, tree)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct PlainPrinter(String);

impl PlainPrinter {
    pub fn take(self) -> String {
        self.0
    }
}

impl TreeWalker for PlainPrinter {
    type Err = std::fmt::Error;

    fn token(&mut self, token: &SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err> {
        let text = token.text(tree);
        write!(self.0, "{text}")
    }

    fn start_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        Ok(())
    }
}

pub struct CstPrinter<W> {
    level: isize,
    out: W,
}

impl<W> CstPrinter<W> {
    pub fn new(out: W) -> Self {
        Self { level: -1, out }
    }

    fn indent(&self) -> String {
        // safe because start_tree is always called first, which increments level to 0 on init
        "  ".repeat(self.level as usize)
    }
}

impl<W: Write> TreeWalker for CstPrinter<W> {
    type Err = std::fmt::Error;

    fn token(&mut self, token: &SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err> {
        let text = token.text(tree);
        let indent = self.indent();

        match token.kind(tree) {
            TokenKind::Space => write!(self.out, "{indent}  Space({})\n", text.len()),
            TokenKind::Newline => write!(self.out, "{indent}  Nl({})\n", text.len()),
            _ => write!(self.out, "{indent}  '{text}'\n"),
        }
    }

    fn start_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.level += 1;

        let indent = self.indent();
        write!(self.out, "{indent}{:?}\n", node.kind(tree))
    }

    fn end_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.level -= 1;
        Ok(())
    }
}

/// Hold a token and the nearest tree kind its under
pub struct TokenContext {
    pub token: Token,
    pub tree_kind: Option<NodeKind>,
}

impl Display for TokenContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = self.tree_kind {
            write!(f, "{kind:?}({})", self.token)
        } else {
            write!(f, "{}", self.token)
        }
    }
}

pub struct LookupSpan {
    target: TextRange,
    current_tree_kind: Option<NodeKind>,
}

impl LookupSpan {
    pub fn new(target: TextRange) -> Self {
        Self {
            target,
            current_tree_kind: None,
        }
    }
}

impl TreeWalker for LookupSpan {
    type Err = Token;

    fn token(&mut self, token: &SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err> {
        let range = token.range(tree);
        if range.contains_range(self.target) {
            return Err(Token {
                kind: token.kind(tree),
                range,
            });
        }

        return Ok(());
    }

    fn start_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.current_tree_kind = Some(node.kind(tree));
        Ok(())
    }

    fn end_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.current_tree_kind = None;
        Ok(())
    }
}
