use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
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

    fn lex(input: &str) -> Vec<TokenKind> {
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
            lex(input),
            [
                Newline, Hash, Space, Ident, Newline, Integer, Space, X, Integer, Newline, Hash,
                Space, Ident, Newline, Bodyweight, Space, X, Integer, Newline, Hash, Space, Ident,
                Newline, Bodyweight, Space, Integer, Second, Newline, Space
            ]
        )
    }

    #[test]
    fn lex_bodyweight() {
        assert_eq!(lex("bw"), [Bodyweight]);
        assert_eq!(lex("BW"), [Bodyweight]);
    }

    #[test]
    fn lex_x() {
        assert_eq!(lex("x"), [X]);
        assert_eq!(lex("X"), [X]);
    }

    #[test]
    fn lex_time_units() {
        assert_eq!(lex("h"), [Hour]);
        assert_eq!(lex("H"), [Hour]);

        assert_eq!(lex("m"), [Minute]);
        assert_eq!(lex("M"), [Minute]);

        assert_eq!(lex("s"), [Second]);
        assert_eq!(lex("S"), [Second]);
    }

    #[test]
    fn lex_colon() {
        assert_eq!(lex(":"), [Colon]);
    }

    #[test]
    fn lex_integer() {
        assert_eq!(lex("1"), [Integer]);
        assert_eq!(lex("123"), [Integer]);
    }

    #[test]
    fn lex_float() {
        assert_eq!(lex("0.42"), [Float]);
        assert_eq!(lex(".42"), [Float]);
        assert_eq!(lex("42.46"), [Float]);
        assert_eq!(lex("42."), [Float]);
    }

    #[test]
    fn lex_hash() {
        assert_eq!(lex("#"), [Hash])
    }

    #[test]
    fn lex_comma() {
        assert_eq!(lex(","), [Comma])
    }

    #[test]
    fn lex_newline() {
        assert_eq!(lex("\n"), [Newline]);
        assert_eq!(lex("\n\n\n\n"), [Newline]);
    }

    #[test]
    fn lex_space() {
        assert_eq!(lex(" "), [Space]);
        assert_eq!(lex("         "), [Space]);
    }

    #[test]
    fn lex_ident() {
        assert_eq!(lex("Bench Press"), [Ident]);
        assert_eq!(lex("Bench    Press"), [Ident]);
        assert_eq!(lex("Bench    Press    "), [Ident, Space]);
    }
}
