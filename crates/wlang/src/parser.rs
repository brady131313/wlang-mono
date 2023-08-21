use std::cell::Cell;

use crate::{
    ast::{Child, Token, Tree, TreeKind},
    lexer::TokenKind,
    utils::TokenSet,
};

enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}

struct MarkOpened {
    index: usize,
}

struct MarkClosed {
    index: usize,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParseError {
    token_idx: usize,
    kind: ParseErrorKind,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParseErrorKind {
    Expected(TokenKind),
    ExpectedOneOf(TokenSet),
    Custom(String),
    UnexpectedEof,
}

impl ParseError {
    pub fn custom(idx: usize, msg: String) -> Self {
        Self {
            token_idx: idx,
            kind: ParseErrorKind::Custom(msg),
        }
    }

    pub fn expected(idx: usize, kind: TokenKind) -> Self {
        Self {
            token_idx: idx,
            kind: ParseErrorKind::Expected(kind),
        }
    }

    pub fn expected_one_of(idx: usize, set: TokenSet) -> Self {
        Self {
            token_idx: idx,
            kind: ParseErrorKind::ExpectedOneOf(set),
        }
    }

    pub fn unexpected_eof(idx: usize) -> Self {
        Self {
            token_idx: idx,
            kind: ParseErrorKind::UnexpectedEof,
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

pub fn parse(tokens: Vec<Token>) -> (Tree, Vec<ParseError>) {
    let mut p = Parser::new(tokens);
    workout(&mut p);

    p.build_tree()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            fuel: Cell::new(256),
            events: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: TreeKind::Error,
        });
        mark
    }

    fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        let mark = MarkOpened { index: m.index };

        // TODO: use index based linked list
        self.events.insert(
            m.index,
            Event::Open {
                kind: TreeKind::Error,
            },
        );
        mark
    }

    fn close(&mut self, m: MarkOpened, kind: TreeKind) -> MarkClosed {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("Parser is stuck")
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(TokenKind::Eof, |it| it.kind)
    }

    /// check next token
    fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    fn at_any(&self, set: TokenSet) -> bool {
        set.is_set(self.nth(0))
    }

    /// at plus consume next token
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_any(&mut self, set: TokenSet) -> bool {
        if self.at_any(set) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// eat plus error reporting
    fn expect(&mut self, kind: TokenKind) -> bool {
        if self.eat(kind) {
            return true;
        }

        self.errors.push(ParseError::expected(self.pos, kind));
        false
    }

    fn expect_and_skip_till(&mut self, kind: TokenKind, skip_till: TokenSet) -> bool {
        if self.expect(kind) {
            return true;
        }

        let m = self.open();
        while !self.at_any(skip_till) && !self.eof() {
            self.advance();
        }
        self.close(m, TreeKind::Error);

        false
    }

    fn expect_any(&mut self, set: TokenSet) -> bool {
        if self.eat_any(set) {
            return true;
        }

        self.errors.push(ParseError::expected_one_of(self.pos, set));
        false
    }

    fn advance_with_error(&mut self, error: &str) {
        if self.eof() {
            self.errors.push(ParseError::unexpected_eof(self.pos));
            return;
        }

        let m = self.open();
        self.errors
            .push(ParseError::custom(self.pos, format!("{error}")));
        self.advance();

        self.close(m, TreeKind::Error);
    }

    const WHITESPACE: TokenSet = TokenSet::from_array([TokenKind::Space, TokenKind::Newline]);

    fn eat_ws(&mut self) {
        while self.at_any(Self::WHITESPACE) {
            self.advance()
        }
    }

    fn build_tree(self) -> (Tree, Vec<ParseError>) {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::new();
        let errors = self.errors;

        // pop the last 'Close' event to ensure that the stack is non-empty in loop
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                // Starting new node; just push empty tree to stack
                Event::Open { kind } => {
                    stack.push(Tree {
                        kind,
                        children: Vec::new(),
                    });
                }
                // tree is done; pop it off the stack and append to new current tree
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Tree(tree));
                }
                // consume token and append to current tree
                Event::Advance => {
                    let token = tokens.next().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
            }
        }

        assert!(stack.len() == 1);
        assert!(tokens.next().is_none());

        (stack.pop().unwrap(), errors)
    }
}

fn workout(p: &mut Parser) {
    let m = p.open();

    while !p.eof() {
        p.eat_ws();

        if p.at(TokenKind::Hash) {
            set_group(p)
        } else {
            p.advance_with_error("expected a set group");
        }
    }

    p.close(m, TreeKind::Workout);
}

const SET_FIRST: TokenSet = WEIGHT_FIRST.with_kind(TokenKind::X);

fn set_group(p: &mut Parser) {
    assert!(p.at(TokenKind::Hash));
    let m = p.open();

    let e = p.open();
    p.expect(TokenKind::Hash);
    p.eat(TokenKind::Space);
    p.expect(TokenKind::Ident);
    p.close(e, TreeKind::Exercise);

    p.eat(TokenKind::Space);
    p.expect(TokenKind::Newline);

    while !p.at(TokenKind::Hash) && !p.eof() {
        p.eat_ws();

        if p.at_any(SET_FIRST) {
            set(p);

            if !p.eof() {
                p.expect(TokenKind::Newline);
            }
        } else {
            break;
        }
    }

    p.close(m, TreeKind::SetGroup);
}

const WEIGHT_FIRST: TokenSet =
    TokenSet::from_array([TokenKind::Float, TokenKind::Integer, TokenKind::Bodyweight]);

const QUANTITY_FIRST: TokenSet = TokenSet::from_array([TokenKind::Integer, TokenKind::X]);
const QUANTITY_END: TokenSet = TokenSet::from_array([
    TokenKind::Second,
    TokenKind::Minute,
    TokenKind::Hour,
    TokenKind::Colon,
    TokenKind::X,
]);

fn set(p: &mut Parser) {
    assert!(p.at_any(SET_FIRST));
    let m = p.open();

    // if weight is followed by a token that can end a quantity
    if p.at_any(WEIGHT_FIRST) && !QUANTITY_END.is_set(p.nth(1)) {
        // weight then quantity
        weight(p);

        p.eat(TokenKind::Space);

        if p.at_any(QUANTITY_FIRST) {
            quantity(p);
        } else if !p.eof() {
            p.advance_with_error("expected quantity");
        }
    } else if p.at_any(QUANTITY_FIRST) {
        // quantity then weight
        quantity(p);

        p.eat(TokenKind::Space);

        if p.at_any(WEIGHT_FIRST) {
            weight(p);
        } else if !p.eof() {
            p.advance_with_error("expected weight");
        }
    }

    // consume trailing spaces
    p.eat(TokenKind::Space);

    p.close(m, TreeKind::Set);
}

fn weight(p: &mut Parser) {
    assert!(p.at_any(WEIGHT_FIRST));
    let m = p.open();

    p.eat_any(WEIGHT_FIRST);
    p.eat(TokenKind::Space);

    if p.at(TokenKind::Plus) {
        p.eat(TokenKind::Plus);
        p.eat(TokenKind::Space);
        p.expect_any(WEIGHT_FIRST);
    }

    p.close(m, TreeKind::Weight);
}

const SIMPLE_DURATION_UNIT: TokenSet =
    TokenSet::from_array([TokenKind::Second, TokenKind::Minute, TokenKind::Hour]);

const REP_RECOVERY: TokenSet = TokenSet::from_array([TokenKind::Newline, TokenKind::Comma]);

fn quantity(p: &mut Parser) {
    assert!(p.at_any(QUANTITY_FIRST));
    let m = p.open();
    let mut typ = TreeKind::Reps;

    if p.at(TokenKind::X) {
        // rep prefix
        p.eat(TokenKind::X);
        p.expect_and_skip_till(TokenKind::Integer, REP_RECOVERY);
    } else if p.at(TokenKind::Integer) {
        // rep suffix
        p.eat(TokenKind::Integer);

        if p.at(TokenKind::X) {
            p.eat(TokenKind::X);
        } else if p.at_any(SIMPLE_DURATION_UNIT) {
            typ = TreeKind::SimpleDuration;
            p.eat_any(SIMPLE_DURATION_UNIT);
        } else if p.at(TokenKind::Colon) {
            // seconds or minutes
            typ = TreeKind::LongDuration;
            p.eat(TokenKind::Colon);
            p.expect(TokenKind::Integer);

            // seconds
            if p.at(TokenKind::Colon) {
                p.eat(TokenKind::Colon);
                p.expect(TokenKind::Integer);
            }
        }
    }

    p.close(m, typ);
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    use super::*;

    macro_rules! parse_snapshot {
        ($input:expr) => {{
            parse_snapshot!($input, [])
        }};

        ($input:expr, $errors:expr) => {{
            let tokens = lex($input);
            let (tree, errors) = parse(tokens);
            let source_tree = crate::ast::SourceTree::new($input, &tree);

            insta::with_settings!({
                description => $input,
                omit_expression => true
            }, {
                insta::assert_debug_snapshot!(source_tree);
            });
            assert_eq!(errors, $errors);
        }};
    }

    #[test]
    fn workout_simplest() {
        parse_snapshot!(
            "
# Bench Press
225 x5"
        );
    }

    #[test]
    fn workout_multiple_sets() {
        parse_snapshot!(
            "
# Bench Press
225 x5
245 x8"
        );
    }

    #[test]
    fn workout_multiple_groups() {
        parse_snapshot!(
            "
# Bench Press
225 x5
245.5 x8

# Pull-ups
bw x5
BW x10"
        );
    }

    #[test]
    fn workout_space_before_set_group() {
        parse_snapshot!(
            "
        # Bench Press
225 x5

    # Pull-ups
bw x3"
        );
    }

    #[test]
    fn workout_space_before_set() {
        parse_snapshot!(
            "
# Bench Press
        225 x5

    245 x8"
        );
    }

    #[test]
    fn workout_space_after_set() {
        parse_snapshot!(
            "
# Bench Press
225 x5      
245 x8      "
        );
    }

    #[test]
    fn workout_reps_x_suffix_and_prefix() {
        parse_snapshot!("#Bench Press\n225 x5");
        parse_snapshot!("#Bench Press\n225 5x");
    }

    #[test]
    fn workout_set_weight_and_quantity_any_order() {
        parse_snapshot!("#Bench Press\n225 x5");
        parse_snapshot!("#Bench Press\nx5 225");

        parse_snapshot!("#Bench Press\n5x 225");

        parse_snapshot!("#Bench Press\nbw 30s");
        parse_snapshot!("#Bench Press\n30s bw");

        parse_snapshot!("#Bench Press\nbw 1:30");
        parse_snapshot!("#Bench Press\n1:30 bw");

        parse_snapshot!("#Pull-ups\nbw + 10 x10");
        parse_snapshot!("#Pull-ups\n10x bw + 10");
    }

    #[test]
    fn workout_simple_duration() {
        parse_snapshot!("#Planks\nbw 30s");
        parse_snapshot!("#Planks\nbw 30m");
        parse_snapshot!("#Planks\nbw 30h");
    }

    #[test]
    fn workout_long_duration() {
        parse_snapshot!("#Planks\nbw 1:30");
        parse_snapshot!("#Planks\nbw 1:30:25");
    }

    #[test]
    fn workout_invalid_rep() {
        parse_snapshot!(
            "#Bench Press\n225 xbench",
            [ParseError::expected(6, TokenKind::Integer)]
        );
    }

    #[test]
    fn fuzz_only_newline() {
        parse_snapshot!("\n", [ParseError::unexpected_eof(1)]);
    }
}
