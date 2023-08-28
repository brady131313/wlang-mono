use aho_corasick::AhoCorasick;
use eventree::TextRange;
use once_cell::sync::Lazy;
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

pub trait CompletionEntry<'e> {
    fn entry(self) -> (&'e str, Completion);
}

impl<'e> CompletionEntry<'e> for &'e str {
    fn entry(self) -> (&'e str, Completion) {
        (self, Completion::Global)
    }
}

#[derive(Default, Debug)]
pub struct CompletionTrie {
    exercises: Trie<String, Completion>,
}

impl CompletionTrie {
    pub fn insert_exercises<'e, I, C>(&mut self, exercises: I)
    where
        I: IntoIterator<Item = C>,
        C: CompletionEntry<'e>,
    {
        for exercise in exercises {
            let (exercise, completion) = exercise.entry();
            let normalized = normalize_exercise(exercise);
            self.exercises.insert(normalized, completion);
        }
    }

    pub fn complete_exercise<'t>(
        &'t self,
        exercise: &str,
    ) -> impl Iterator<Item = (String, Completion)> + 't {
        let normalized = normalize_exercise(exercise);

        self.exercises
            .get_raw_descendant(&normalized)
            .into_iter()
            .flat_map(|trie| trie.iter())
            .map(|(exercise, &completion)| (denormalize_exercise(exercise), completion))
    }
}

fn normalize_exercise(exercise: &str) -> String {
    let mut output = String::new();

    // remove ws and lowercase
    let mut last_char_ws = false;
    for c in exercise.chars() {
        if c.is_whitespace() {
            if !last_char_ws {
                output.push('_');
            }
            last_char_ws = true;
        } else if c == '-' {
            output.push('_');
        } else {
            output.extend(c.to_lowercase());
            last_char_ws = false;
        }
    }

    // transform common prefixes
    const REPLACEMENT: &[&str] = &["db", "sl", "sa"];
    static AC_NORMALIZE: Lazy<AhoCorasick> = Lazy::new(|| {
        const PATTERNS: &[&str] = &["dumbbell", "single_leg", "single_arm"];
        AhoCorasick::new(PATTERNS).unwrap()
    });

    AC_NORMALIZE.replace_all(&output, REPLACEMENT)
}

fn denormalize_exercise(exercise: &str) -> String {
    // transform common prefixes
    const REPLACEMENTS: &[&str] = &["DB", "SL", "SA"];
    static AC_DENORMALIZE: Lazy<AhoCorasick> = Lazy::new(|| {
        const PATTERNS: &[&str] = &["db", "sl", "sa"];
        AhoCorasick::new(PATTERNS).unwrap()
    });
    let replaced = AC_DENORMALIZE.replace_all(exercise, REPLACEMENTS);

    // re add spaces and capitalize
    let mut output = String::new();
    let mut last_char_ws = true;
    for c in replaced.chars() {
        if c == '_' {
            output.push(' ');
            last_char_ws = true;
        } else {
            if last_char_ws {
                output.extend(c.to_uppercase());
            } else {
                output.push(c);
            }
            last_char_ws = false;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_exercise_lowercases_and_removes_spaces() {
        assert_eq!(normalize_exercise("Exercise  Here"), "exercise_here");
        assert_eq!(normalize_exercise("Exercise-Here"), "exercise_here");
    }

    #[test]
    fn normalize_exercise_transforms_common_abbreviations() {
        assert_eq!(normalize_exercise("Dumbbell Bench Press"), "db_bench_press");

        assert_eq!(normalize_exercise("Single Leg Squat"), "sl_squat");
        assert_eq!(normalize_exercise("Single-Leg Squat"), "sl_squat");

        assert_eq!(normalize_exercise("Single Arm Curl"), "sa_curl");
        assert_eq!(normalize_exercise("Single-Arm Curl"), "sa_curl");
    }

    #[test]
    fn denormalize_exercise_capitalizes_and_adds_spaces() {
        assert_eq!(denormalize_exercise("exercise_here"), "Exercise Here")
    }

    #[test]
    fn denormalize_exercise_transforms_common_abbreviations() {
        assert_eq!(denormalize_exercise("sl_squat"), "SL Squat");
        assert_eq!(denormalize_exercise("sa_curl"), "SA Curl");
        assert_eq!(denormalize_exercise("db_row"), "DB Row");
    }

    #[test]
    fn completion_trie_exercises() {
        let mut trie = CompletionTrie::default();
        trie.insert_exercises([
            "Bench Press",
            "Overhead Press",
            "Pull-ups",
            "DB Bench",
            "DB Incline Bench",
            "DB Row",
            "DB Curl",
        ]);

        let completions: Vec<_> = trie.complete_exercise("dumbbell").map(|(e, _)| e).collect();
        let completions_abbr: Vec<_> = trie.complete_exercise("DB").map(|(e, _)| e).collect();

        assert_eq!(
            completions,
            ["DB Bench", "DB Curl", "DB Incline Bench", "DB Row"]
        );
        assert_eq!(completions, completions_abbr);
    }
}
