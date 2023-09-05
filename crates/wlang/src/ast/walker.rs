use std::fmt::Write;

use eventree::{TextRange, TextSize};

use crate::lexer::TokenKind;

use super::{SyntaxNode, SyntaxToken, SyntaxTree};

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
            Some(TokenContext {
                token,
                node: walker.current_node,
            })
        } else {
            None
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
        walker.start_tree(self, tree)?;

        for child in self.children(tree) {
            match child {
                eventree::SyntaxElement::Node(node) => node.walk(walker, tree)?,
                eventree::SyntaxElement::Token(token) => walker.token(&token, tree)?,
            }
        }

        walker.end_tree(self, tree)?;

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

    fn start_tree(&mut self, _node: &SyntaxNode, _tree: &SyntaxTree) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(&mut self, _node: &SyntaxNode, _tree: &SyntaxTree) -> Result<(), Self::Err> {
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
            TokenKind::Space => writeln!(self.out, "{indent}  Space({})", text.len()),
            TokenKind::Newline => writeln!(self.out, "{indent}  Nl({})", text.len()),
            _ => writeln!(self.out, "{indent}  '{text}'"),
        }
    }

    fn start_tree(&mut self, node: &SyntaxNode, tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.level += 1;

        let indent = self.indent();
        writeln!(self.out, "{indent}{:?}", node.kind(tree))
    }

    fn end_tree(&mut self, _node: &SyntaxNode, _tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.level -= 1;
        Ok(())
    }
}

/// Hold a token and the nearest tree kind its under
pub struct TokenContext {
    pub token: SyntaxToken,
    pub node: Option<SyntaxNode>,
}

pub struct LookupSpan {
    target: TextRange,
    current_node: Option<SyntaxNode>,
}

impl LookupSpan {
    pub fn new(target: TextRange) -> Self {
        Self {
            target,
            current_node: None,
        }
    }
}

impl TreeWalker for LookupSpan {
    type Err = SyntaxToken;

    fn token(&mut self, token: &SyntaxToken, tree: &SyntaxTree) -> Result<(), Self::Err> {
        let range = token.range(tree);
        if range.contains_range(self.target) {
            return Err(*token);
        }

        Ok(())
    }

    fn start_tree(&mut self, node: &SyntaxNode, _tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.current_node = Some(*node);
        Ok(())
    }

    fn end_tree(&mut self, _node: &SyntaxNode, _tree: &SyntaxTree) -> Result<(), Self::Err> {
        self.current_node = None;
        Ok(())
    }
}
