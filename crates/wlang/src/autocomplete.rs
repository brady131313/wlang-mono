use crate::{ast::walker::TreeWalker, lexer::TokenKind};

pub struct SemanticTokenCollector {
    tokens: Vec<String>,
}

impl TreeWalker for SemanticTokenCollector {
    type Err = ();

    fn token(
        &mut self,
        token: &crate::ast::SyntaxToken,
        tree: &crate::ast::SyntaxTree,
    ) -> Result<(), Self::Err> {
        if token.kind(tree) == TokenKind::Ident {
            self.tokens.push(token.text(tree).to_string())
        }

        Ok(())
    }

    fn start_tree(
        &mut self,
        node: &crate::ast::SyntaxNode,
        tree: &crate::ast::SyntaxTree,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(
        &mut self,
        node: &crate::ast::SyntaxNode,
        tree: &crate::ast::SyntaxTree,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}

pub struct CompletionTrie {}

impl CompletionTrie {
    pub fn new(initial_tokens: Vec<String>) -> Self {
        todo!("Radix trie crate to build lookup tree")
    }
}
