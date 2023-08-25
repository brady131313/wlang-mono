use std::fmt::{Debug, Display, Write};

use text_size::{TextRange, TextSize};

use crate::lexer::TokenKind;

use super::{Child, SourceTree, Token, Tree, TreeKind};

pub trait TreeWalker {
    type Err;

    fn token(&mut self, token: &Token, source: &str) -> Result<(), Self::Err>;

    fn start_tree(&mut self, kind: TreeKind) -> Result<(), Self::Err>;

    fn end_tree(&mut self, kind: TreeKind) -> Result<(), Self::Err>;
}

impl Tree {
    pub fn walk<W: TreeWalker>(&self, walker: &mut W, source: &str) -> Result<(), W::Err> {
        walker.start_tree(self.kind)?;

        for child in &self.children {
            match child {
                Child::Token(token) => walker.token(token, source)?,
                Child::Tree(tree) => tree.walk(walker, source)?,
            }
        }

        walker.end_tree(self.kind)?;
        Ok(())
    }

    pub fn lookup_span(&self, span: TextRange, source: &str) -> Option<TokenContext> {
        let mut walker = LookupSpan::new(span);

        if let Err(token) = self.walk(&mut walker, source) {
            return Some(TokenContext {
                token,
                tree_kind: walker.current_tree_kind,
            });
        } else {
            return None;
        }
    }

    pub fn lookup_offset(&self, offset: impl Into<TextSize>, source: &str) -> Option<TokenContext> {
        let span = TextRange::empty(offset.into());
        self.lookup_span(span, source)
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

    fn token(&mut self, token: &Token, source: &str) -> Result<(), Self::Err> {
        let text = &source[token.span];
        write!(self.0, "{text}")
    }

    fn start_tree(&mut self, _kind: TreeKind) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(&mut self, _kind: TreeKind) -> Result<(), Self::Err> {
        Ok(())
    }
}

struct CstPrinter<W> {
    level: isize,
    out: W,
}

impl<W> CstPrinter<W> {
    fn new(out: W) -> Self {
        Self { level: -1, out }
    }

    fn indent(&self) -> String {
        // safe because start_tree is always called first, which increments level to 0 on init
        "  ".repeat(self.level as usize)
    }
}

impl<W: Write> TreeWalker for CstPrinter<W> {
    type Err = std::fmt::Error;

    fn token(&mut self, token: &Token, source: &str) -> Result<(), Self::Err> {
        let text = &source[token.span];
        let indent = self.indent();

        match token.kind {
            TokenKind::Space => write!(self.out, "{indent}  Space({})\n", text.len()),
            TokenKind::Newline => write!(self.out, "{indent}  Nl({})\n", text.len()),
            _ => write!(self.out, "{indent}  '{text}'\n"),
        }
    }

    fn start_tree(&mut self, kind: TreeKind) -> Result<(), Self::Err> {
        self.level += 1;

        let indent = self.indent();
        write!(self.out, "{indent}{:?}\n", kind)
    }

    fn end_tree(&mut self, _kind: TreeKind) -> Result<(), Self::Err> {
        self.level -= 1;
        Ok(())
    }
}

impl<'i> Debug for SourceTree<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cst_printer = CstPrinter::new(f);
        self.tree.walk(&mut cst_printer, self.source)
    }
}

/// Hold a token and the nearest tree kind its under
pub struct TokenContext {
    pub token: Token,
    pub tree_kind: Option<TreeKind>,
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
    current_tree_kind: Option<TreeKind>,
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
    /// use the Err variant to exit lookup early once we found containing span
    type Err = Token;

    fn token(&mut self, token: &Token, _source: &str) -> Result<(), Self::Err> {
        if token.span.contains_range(self.target) {
            // stop walking tree, found match
            return Err(*token);
        }

        return Ok(());
    }

    fn start_tree(&mut self, kind: TreeKind) -> Result<(), Self::Err> {
        self.current_tree_kind = Some(kind);
        Ok(())
    }

    fn end_tree(&mut self, _kind: TreeKind) -> Result<(), Self::Err> {
        self.current_tree_kind = None;
        Ok(())
    }
}
