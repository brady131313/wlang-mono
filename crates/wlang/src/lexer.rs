use std::fmt::Display;

use eventree::{TextRange, TextSize};
use logos::{Lexer, Logos, Source};

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(input);
    let mut tokens = Vec::new();
    while let Some(kind) = lexer.next() {
        let span = lexer.span();
        tokens.push(Token {
            kind: kind.unwrap_or(TokenKind::Error),
            range: TextRange::new(
                TextSize::new(span.start as u32),
                TextSize::new(span.end as u32),
            ),
        });
    }

    tokens
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub range: TextRange,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.range)
    }
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum TokenKind {
    #[token("bw", ignore(ascii_case))]
    Bodyweight,
    #[token("x", ignore(ascii_case))]
    X,
    #[token("+")]
    Plus,

    #[regex("[0-9]+")]
    Integer,
    #[regex(r#"([1-9][0-9]*|0)?\.[0-9]*"#)]
    Float,

    #[token("h", ignore(ascii_case))]
    Hour,
    #[token("m", ignore(ascii_case))]
    Minute,
    #[token("s", ignore(ascii_case))]
    Second,
    #[token(":")]
    Colon,

    #[token("#")]
    Hash,
    #[token(",")]
    Comma,
    #[regex("\n+")]
    Newline,
    #[regex("[ \t]+")]
    Space,
    #[regex("[hHmMsSxD][a-zA-Z]", ident)]
    #[regex("[a-zA-Z]", ident)]
    Ident,

    Eof,

    // must always be last variant
    Error,
}

impl TokenKind {
    pub fn is_whitespace(self) -> bool {
        matches!(self, TokenKind::Space | TokenKind::Newline)
    }
}

fn ident(lex: &mut Lexer<TokenKind>) {
    let mut last_significant = 0;
    let remaining = lex.remainder();

    for (i, c) in remaining.char_indices() {
        if matches!(c, ',' | '\n') {
            break;
        }

        if !matches!(c, ' ' | '\t') {
            last_significant = remaining.find_boundary(i + 1);
        }
    }

    lex.bump(last_significant);
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    fn lex_kind(input: &str) -> Vec<TokenKind> {
        let lex = TokenKind::lexer(input);
        lex.into_iter().map(Result::unwrap).collect()
    }

    #[test]
    fn lex_it_all() {
        let input = "
# Bench Press
225 x5

# Pull-ups
BW x5

# Planks
bw 30s
            ";

        assert_eq!(
            lex_kind(input),
            [
                Newline, Hash, Space, Ident, Newline, Integer, Space, X, Integer, Newline, Hash,
                Space, Ident, Newline, Bodyweight, Space, X, Integer, Newline, Hash, Space, Ident,
                Newline, Bodyweight, Space, Integer, Second, Newline, Space
            ]
        )
    }

    #[test]
    fn lex_bodyweight() {
        assert_eq!(lex_kind("bw"), [Bodyweight]);
        assert_eq!(lex_kind("BW"), [Bodyweight]);
    }

    #[test]
    fn lex_x() {
        assert_eq!(lex_kind("x"), [X]);
        assert_eq!(lex_kind("X"), [X]);
    }

    #[test]
    fn lex_plus() {
        assert_eq!(lex_kind("+"), [Plus]);
    }

    #[test]
    fn lex_time_units() {
        assert_eq!(lex_kind("h"), [Hour]);
        assert_eq!(lex_kind("H"), [Hour]);

        assert_eq!(lex_kind("m"), [Minute]);
        assert_eq!(lex_kind("M"), [Minute]);

        assert_eq!(lex_kind("s"), [Second]);
        assert_eq!(lex_kind("S"), [Second]);
    }

    #[test]
    fn lex_colon() {
        assert_eq!(lex_kind(":"), [Colon]);
    }

    #[test]
    fn lex_integer() {
        assert_eq!(lex_kind("1"), [Integer]);
        assert_eq!(lex_kind("123"), [Integer]);
        assert_eq!(lex_kind("00"), [Integer]);
    }

    #[test]
    fn lex_float() {
        assert_eq!(lex_kind("0.42"), [Float]);
        assert_eq!(lex_kind(".42"), [Float]);
        assert_eq!(lex_kind("42.46"), [Float]);
        assert_eq!(lex_kind("42."), [Float]);
    }

    #[test]
    fn lex_hash() {
        assert_eq!(lex_kind("#"), [Hash])
    }

    #[test]
    fn lex_comma() {
        assert_eq!(lex_kind(","), [Comma])
    }

    #[test]
    fn lex_newline() {
        assert_eq!(lex_kind("\n"), [Newline]);
        assert_eq!(lex_kind("\n\n\n\n"), [Newline]);
    }

    #[test]
    fn lex_space() {
        assert_eq!(lex_kind(" "), [Space]);
        assert_eq!(lex_kind("         "), [Space]);
    }

    #[test]
    fn lex_ident() {
        assert_eq!(lex_kind("Bench Press"), [Ident]);
        assert_eq!(lex_kind("Bench    Press"), [Ident]);
        assert_eq!(lex_kind("Bench    Press    "), [Ident, Space]);
    }

    #[test]
    fn lex_ident_starting_with_keyword() {
        assert_eq!(lex_kind("Squat"), [Ident]);
        assert_eq!(lex_kind("# Squat"), [Hash, Space, Ident])
    }

    #[test]
    fn fuzz_cases() {
        assert_eq!(lex_kind("OӴ"), [Ident]);
    }
}
