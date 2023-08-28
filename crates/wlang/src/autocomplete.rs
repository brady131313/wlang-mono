use eventree::TextRange;
use radix_trie::{Trie, TrieCommon};

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
        _node: &crate::ast::SyntaxNode,
        _tree: &crate::ast::SyntaxTree,
    ) -> Result<(), Self::Err> {
        Ok(())
    }

    fn end_tree(
        &mut self,
        _node: &crate::ast::SyntaxNode,
        _tree: &crate::ast::SyntaxTree,
    ) -> Result<(), Self::Err> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completion {
    Global,
    Local(TextRange),
}

pub trait CompletionEntry {
    fn entry(self) -> (String, Completion);
}

impl CompletionEntry for String {
    fn entry(self) -> (String, Completion) {
        (self, Completion::Global)
    }
}

#[derive(Default, Debug)]
pub struct CompletionTrie {
    exercises: Trie<String, Completion>,
}

impl CompletionTrie {
    pub fn insert_exercises<I, C>(&mut self, exercises: I)
    where
        I: IntoIterator<Item = C>,
        C: CompletionEntry,
    {
        for exercise in exercises {
            let (exercise, completion) = exercise.entry();
            self.exercises.insert(exercise, completion);
        }
    }

    pub fn complete_exercise<'t>(
        &'t self,
        exercise: &str,
    ) -> impl Iterator<Item = (&'t String, Completion)> {
        self.exercises
            .get_raw_descendant(exercise)
            .into_iter()
            .flat_map(|trie| trie.iter())
            .map(|(exercise, &completion)| (exercise, completion))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_trie_exercises() {
        let mut trie = CompletionTrie::default();
        trie.insert_exercises([
            String::from("Bench Press"),
            String::from("Overhead Press"),
            String::from("Pull-ups"),
            String::from("DB Bench"),
            String::from("DB Incline Bench"),
            String::from("DB Row"),
            String::from("DB Curl"),
        ]);

        let completions: Vec<_> = trie.complete_exercise("DB").map(|(e, _)| e).collect();
        assert_eq!(
            completions,
            ["DB Bench", "DB Curl", "DB Incline Bench", "DB Row"]
        );
    }
}
