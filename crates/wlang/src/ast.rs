use text_size::TextRange;

use crate::lexer::TokenKind;
use std::fmt::{Debug, Display};

use self::walker::TreeWalker;

pub mod walker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeKind {
    Error,
    Workout,
    Exercise,
    SetGroup,
    Set,
    Weight,
    Reps,
    SimpleDuration,
    LongDuration,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextRange,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.span)
    }
}

pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Child>,
}

pub struct SourceTree<'i> {
    source: &'i str,
    tree: &'i Tree,
}

impl<'i> SourceTree<'i> {
    pub fn new(source: &'i str, tree: &'i Tree) -> Self {
        Self { source, tree }
    }
}

pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Child {
    fn into_token(&self) -> Option<&Token> {
        if let Self::Token(token) = self {
            Some(token)
        } else {
            None
        }
    }

    fn into_tree(&self) -> Option<&Tree> {
        if let Self::Tree(tree) = self {
            Some(tree)
        } else {
            None
        }
    }
}

pub trait AstTree<'i>: Sized {
    fn cast(tree: &'i Tree) -> Option<Self>;
}

pub trait AstToken<'i>: Sized {
    fn cast(token: &'i Token) -> Option<Self>;

    fn text<'s>(&self, source: &'s str) -> &'s str;

    fn span(&self) -> TextRange;
}

macro_rules! impl_ast_tree {
    ($enum_name:ident::$variant:ident) => {
        pub struct $variant<'i>(&'i Tree);

        impl<'i> AstTree<'i> for $variant<'i> {
            fn cast(tree: &'i Tree) -> Option<Self> {
                if tree.kind == $enum_name::$variant {
                    Some(Self(tree))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! impl_ast_token {
    ($enum_name:ident::$variant:ident) => {
        pub struct $variant<'i>(&'i Token);

        impl<'i> AstToken<'i> for $variant<'i> {
            fn cast(token: &'i Token) -> Option<Self> {
                if token.kind == $enum_name::$variant {
                    Some(Self(token))
                } else {
                    None
                }
            }

            fn text<'s>(&self, source: &'s str) -> &'s str {
                &source[self.span()]
            }

            fn span(&self) -> TextRange {
                self.0.span
            }
        }
    };
}

fn child_trees<'i, A: AstTree<'i> + 'i>(tree: &'i Tree) -> impl Iterator<Item = A> + 'i {
    tree.children
        .iter()
        .filter_map(Child::into_tree)
        .filter_map(A::cast)
}

fn find_child_tree<'i, A: AstTree<'i> + 'i>(tree: &'i Tree) -> Option<A> {
    child_trees(tree).next()
}

fn child_tokens<'i, A: AstToken<'i> + 'i>(tree: &'i Tree) -> impl Iterator<Item = A> + 'i {
    tree.children
        .iter()
        .filter_map(Child::into_token)
        .filter_map(A::cast)
}

fn find_child_token<'i, A: AstToken<'i> + 'i>(tree: &'i Tree) -> Option<A> {
    child_tokens(tree).next()
}

impl_ast_tree!(TreeKind::Workout);

impl<'i> Workout<'i> {
    pub fn set_groups(&self) -> impl Iterator<Item = SetGroup> {
        child_trees(self.0)
    }

    pub fn walk<W: TreeWalker>(&self, walker: &mut W, source: &str) -> Result<(), W::Err> {
        self.0.walk(walker, source)
    }
}

impl_ast_tree!(TreeKind::SetGroup);

impl<'i> SetGroup<'i> {
    pub fn exercise(&self) -> Option<Exercise> {
        find_child_tree(self.0)
    }

    pub fn sets(&self) -> impl Iterator<Item = Set> {
        child_trees(self.0)
    }
}

impl_ast_tree!(TreeKind::Exercise);

impl<'i> Exercise<'i> {
    pub fn ident(&self) -> Option<Ident> {
        find_child_token(self.0)
    }
}

impl_ast_tree!(TreeKind::Set);

impl<'i> Set<'i> {
    pub fn weight(&self) -> Option<Weight> {
        find_child_tree(self.0)
    }

    pub fn quantity(&self) -> Option<Quantity> {
        find_child_tree(self.0)
    }
}

impl_ast_tree!(TreeKind::Weight);

impl<'i> Weight<'i> {
    pub fn weight(&self) -> Option<WeightLiteral> {
        find_child_token(self.0)
    }

    pub fn bodyweight(&self) -> Option<Bodyweight> {
        find_child_token(self.0)
    }
}

pub enum WeightLiteral<'i> {
    Float(Float<'i>),
    Integer(Integer<'i>),
}

impl<'i> WeightLiteral<'i> {
    pub fn parse(&self, source: &str) -> f64 {
        match self {
            WeightLiteral::Float(float) => float.parse(source),
            WeightLiteral::Integer(int) => int.parse(source) as f64,
        }
    }
}

impl<'i> AstToken<'i> for WeightLiteral<'i> {
    fn cast(token: &'i Token) -> Option<Self> {
        match token.kind {
            TokenKind::Float => Some(Self::Float(Float(token))),
            TokenKind::Integer => Some(Self::Integer(Integer(token))),
            _ => None,
        }
    }

    fn text<'s>(&self, source: &'s str) -> &'s str {
        match self {
            WeightLiteral::Float(float) => float.text(source),
            WeightLiteral::Integer(integer) => integer.text(source),
        }
    }

    fn span(&self) -> TextRange {
        match self {
            WeightLiteral::Float(float) => float.span(),
            WeightLiteral::Integer(integer) => integer.span(),
        }
    }
}

pub enum Quantity<'i> {
    Reps(Reps<'i>),
    SimpleDuration(SimpleDuration<'i>),
    LongDuration(LongDuration<'i>),
}

impl<'i> AstTree<'i> for Quantity<'i> {
    fn cast(tree: &'i Tree) -> Option<Self> {
        match tree.kind {
            TreeKind::Reps => Some(Self::Reps(Reps(tree))),
            TreeKind::SimpleDuration => Some(Self::SimpleDuration(SimpleDuration(tree))),
            TreeKind::LongDuration => Some(Self::LongDuration(LongDuration(tree))),
            _ => None,
        }
    }
}

impl_ast_tree!(TreeKind::Reps);

impl<'i> Reps<'i> {
    pub fn amount(&self) -> Option<Integer> {
        find_child_token(self.0)
    }
}

impl_ast_tree!(TreeKind::SimpleDuration);

impl<'i> SimpleDuration<'i> {
    pub fn duration(&self) -> Option<Integer> {
        find_child_token(self.0)
    }

    pub fn unit(&self) -> Option<TimeUnit> {
        find_child_token(self.0)
    }
}

pub enum TimeUnit<'i> {
    Hour(Hour<'i>),
    Minute(Minute<'i>),
    Second(Second<'i>),
}

impl<'i> AstToken<'i> for TimeUnit<'i> {
    fn cast(token: &'i Token) -> Option<Self> {
        match token.kind {
            TokenKind::Hour => Some(Self::Hour(Hour(token))),
            TokenKind::Minute => Some(Self::Minute(Minute(token))),
            TokenKind::Second => Some(Self::Second(Second(token))),
            _ => None,
        }
    }

    fn text<'s>(&self, source: &'s str) -> &'s str {
        match self {
            TimeUnit::Hour(hour) => hour.text(source),
            TimeUnit::Minute(minute) => minute.text(source),
            TimeUnit::Second(second) => second.text(source),
        }
    }

    fn span(&self) -> TextRange {
        match self {
            TimeUnit::Hour(hour) => hour.span(),
            TimeUnit::Minute(minute) => minute.span(),
            TimeUnit::Second(second) => second.span(),
        }
    }
}

impl_ast_token!(TokenKind::Hour);
impl_ast_token!(TokenKind::Minute);
impl_ast_token!(TokenKind::Second);

impl_ast_tree!(TreeKind::LongDuration);

impl<'i> LongDuration<'i> {
    fn has_hour_comp(&self) -> bool {
        self.0.children.len() >= 4
    }

    pub fn hour(&self) -> Option<Integer> {
        if self.has_hour_comp() {
            find_child_token(self.0)
        } else {
            None
        }
    }

    pub fn minute(&self) -> Option<Integer> {
        if self.has_hour_comp() {
            child_tokens(self.0).nth(1)
        } else {
            find_child_token(self.0)
        }
    }

    pub fn second(&self) -> Option<Integer> {
        if self.has_hour_comp() {
            child_tokens(self.0).nth(2)
        } else {
            child_tokens(self.0).nth(1)
        }
    }
}

impl_ast_token!(TokenKind::Bodyweight);
impl_ast_token!(TokenKind::Ident);

impl_ast_token!(TokenKind::Float);

impl<'i> Float<'i> {
    pub fn parse(&self, source: &str) -> f64 {
        self.text(source).parse().unwrap()
    }
}

impl_ast_token!(TokenKind::Integer);

impl<'i> Integer<'i> {
    pub fn parse(&self, source: &str) -> usize {
        self.text(source).parse().unwrap()
    }
}
