use logos::{Lexer, Logos};

use crate::ast::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = TokenKind::lexer(input);
    let mut tokens = Vec::new();
    while let Some(kind) = lexer.next() {
        tokens.push(Token {
            kind: kind.unwrap_or(TokenKind::Error),
            text: lexer.slice(),
        });
    }

    tokens
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[token("bw", ignore(ascii_case))]
    Bodyweight,
    #[token("x", ignore(ascii_case))]
    X,

    #[regex("[1-9][0-9]*|0")]
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
    #[regex("[a-zA-Z]", ident)]
    Ident,

    Eof,
    Error,
}

impl TokenKind {
    pub fn is_whitespace(self) -> bool {
        matches!(self, TokenKind::Space | TokenKind::Newline)
    }

    pub fn set_start(self) -> bool {
        matches!(
            self,
            TokenKind::Bodyweight | TokenKind::X | TokenKind::Integer | TokenKind::Float
        )
    }

    pub fn is_number(self) -> bool {
        matches!(self, TokenKind::Float | TokenKind::Integer)
    }
}

fn ident(lex: &mut Lexer<TokenKind>) {
    let mut bytes_till_end = 0;
    let mut last_significant = 0;
    for c in lex.remainder().chars() {
        if matches!(c, ',' | '\n') {
            break;
        }
        bytes_till_end += 1;

        if !matches!(c, ' ' | '\t') {
            last_significant = bytes_till_end;
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
}
