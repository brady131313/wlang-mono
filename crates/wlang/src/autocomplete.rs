use crate::{
    ast::{walker::TreeWalker, Token},
    lexer::TokenKind,
};

pub struct SemanticTokenCollector {
    tokens: Vec<String>,
}

impl TreeWalker for SemanticTokenCollector {
    type Err = ();

    fn token(&mut self, token: &Token, source: &str) -> Result<(), Self::Err> {
        if token.kind == TokenKind::Ident {
            self.tokens.push(source[token.span].to_string())
        }

        Ok(())
    }

    fn start_tree(&mut self, kind: crate::ast::TreeKind) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(&mut self, kind: crate::ast::TreeKind) -> Result<(), Self::Err> {
        Ok(())
    }
}

pub struct CompletionTrie {}

impl CompletionTrie {
    pub fn new(initial_tokens: Vec<String>) -> Self {
        todo!("Radix trie crate to build lookup tree")
    }
}
