use crate::lexer::TokenKind;
use std::fmt::{Debug, Write};

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

#[derive(Debug)]
pub struct Token<'i> {
    pub kind: TokenKind,
    pub text: &'i str,
}

pub struct Tree<'i> {
    pub kind: TreeKind,
    pub children: Vec<Child<'i>>,
}

pub enum Child<'i> {
    Token(Token<'i>),
    Tree(Tree<'i>),
}

impl<'i> Child<'i> {
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
    fn cast(tree: &'i Tree<'i>) -> Option<Self>;
}

pub trait AstToken<'i>: Sized {
    fn cast(token: &'i Token<'i>) -> Option<Self>;

    fn text(&self) -> &str;
}

macro_rules! impl_ast_tree {
    ($enum_name:ident::$variant:ident) => {
        #[derive(Debug)]
        pub struct $variant<'i>(&'i Tree<'i>);

        impl<'i> AstTree<'i> for $variant<'i> {
            fn cast(tree: &'i Tree<'i>) -> Option<Self> {
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
        #[derive(Debug)]
        pub struct $variant<'i>(&'i Token<'i>);

        impl<'i> AstToken<'i> for $variant<'i> {
            fn cast(token: &'i Token<'i>) -> Option<Self> {
                if token.kind == $enum_name::$variant {
                    Some(Self(token))
                } else {
                    None
                }
            }

            fn text(&self) -> &str {
                self.0.text
            }
        }
    };
}

fn child_trees<'i, A: AstTree<'i> + 'i>(tree: &'i Tree<'i>) -> impl Iterator<Item = A> + 'i {
    tree.children
        .iter()
        .filter_map(Child::into_tree)
        .filter_map(A::cast)
}

fn find_child_tree<'i, A: AstTree<'i> + 'i>(tree: &'i Tree<'i>) -> Option<A> {
    child_trees(tree).next()
}

fn child_tokens<'i, A: AstToken<'i> + 'i>(tree: &'i Tree<'i>) -> impl Iterator<Item = A> + 'i {
    tree.children
        .iter()
        .filter_map(Child::into_token)
        .filter_map(A::cast)
}

fn find_child_token<'i, A: AstToken<'i> + 'i>(tree: &'i Tree<'i>) -> Option<A> {
    child_tokens(tree).next()
}

impl_ast_tree!(TreeKind::Workout);

impl<'i> Workout<'i> {
    pub fn set_groups(&self) -> impl Iterator<Item = SetGroup> {
        child_trees(self.0)
    }

    pub fn walk<W: TreeWalker>(&self, walker: &mut W) {
        self.0.walk(walker)
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
    pub fn parse(&self) -> f64 {
        match self {
            WeightLiteral::Float(float) => float.parse(),
            WeightLiteral::Integer(int) => int.parse() as f64,
        }
    }
}

impl<'i> AstToken<'i> for WeightLiteral<'i> {
    fn cast(token: &'i Token<'i>) -> Option<Self> {
        match token.kind {
            TokenKind::Float => Some(Self::Float(Float(token))),
            TokenKind::Integer => Some(Self::Integer(Integer(token))),
            _ => None,
        }
    }

    fn text(&self) -> &str {
        match self {
            WeightLiteral::Float(float) => float.text(),
            WeightLiteral::Integer(integer) => integer.text(),
        }
    }
}

pub enum Quantity<'i> {
    Reps(Reps<'i>),
    SimpleDuration(SimpleDuration<'i>),
    LongDuration(LongDuration<'i>),
}

impl<'i> AstTree<'i> for Quantity<'i> {
    fn cast(tree: &'i Tree<'i>) -> Option<Self> {
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
    fn cast(token: &'i Token<'i>) -> Option<Self> {
        match token.kind {
            TokenKind::Hour => Some(Self::Hour(Hour(token))),
            TokenKind::Minute => Some(Self::Minute(Minute(token))),
            TokenKind::Second => Some(Self::Second(Second(token))),
            _ => None,
        }
    }

    fn text(&self) -> &str {
        match self {
            TimeUnit::Hour(hour) => hour.text(),
            TimeUnit::Minute(minute) => minute.text(),
            TimeUnit::Second(second) => second.text(),
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
    pub fn parse(&self) -> f64 {
        self.0.text.parse().unwrap()
    }
}

impl_ast_token!(TokenKind::Integer);

impl<'i> Integer<'i> {
    pub fn parse(&self) -> usize {
        self.0.text.parse().unwrap()
    }
}

pub trait TreeWalker {
    fn token(&mut self, token: &Token);

    fn start_tree(&mut self, kind: TreeKind);

    fn end_tree(&mut self, kind: TreeKind);
}

impl<'i> Tree<'i> {
    fn print<W: Write>(&self, buf: &mut W, level: usize) -> std::fmt::Result {
        let indent = "  ".repeat(level);
        write!(buf, "{indent}{:?}\n", self.kind)?;

        for child in &self.children {
            match child {
                Child::Token(token) => match token.kind {
                    TokenKind::Space => write!(buf, "{indent}  Space({})\n", token.text.len())?,
                    TokenKind::Newline => write!(buf, "{indent}  Nl({})\n", token.text.len())?,
                    _ => write!(buf, "{indent}  '{}'\n", token.text)?,
                },
                Child::Tree(tree) => tree.print(buf, level + 1)?,
            }
        }

        Ok(())
    }

    fn walk<W: TreeWalker>(&self, walker: &mut W) {
        walker.start_tree(self.kind);

        for child in &self.children {
            match child {
                Child::Token(token) => walker.token(token),
                Child::Tree(tree) => tree.walk(walker),
            }
        }

        walker.end_tree(self.kind);
    }
}

impl<'i> Debug for Tree<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f, 0)
    }
}
