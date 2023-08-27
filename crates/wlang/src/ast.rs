use eventree::TextRange;

use crate::lexer::TokenKind;
use std::fmt::Debug;

use self::walker::{CstPrinter, SyntaxNodeExt};

pub mod walker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NodeKind {
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

unsafe impl eventree::SyntaxKind for NodeKind {
    fn to_raw(self) -> u16 {
        self as u16
    }

    unsafe fn from_raw(raw: u16) -> Self {
        std::mem::transmute(raw as u8)
    }
}

unsafe impl eventree::SyntaxKind for TokenKind {
    fn to_raw(self) -> u16 {
        self as u16
    }

    unsafe fn from_raw(raw: u16) -> Self {
        std::mem::transmute(raw as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeConfig {}

impl eventree::TreeConfig for TreeConfig {
    type NodeKind = NodeKind;

    type TokenKind = TokenKind;
}

pub type SyntaxNode = eventree::SyntaxNode<TreeConfig>;
pub type SyntaxToken = eventree::SyntaxToken<TreeConfig>;
pub type SyntaxTree = eventree::SyntaxTree<TreeConfig>;
pub type SyntaxBuilder = eventree::SyntaxBuilder<TreeConfig>;

pub struct SourceTree<'t> {
    tree: &'t SyntaxTree,
}

impl<'t> SourceTree<'t> {
    pub fn new(tree: &'t SyntaxTree) -> Self {
        Self { tree }
    }
}

impl<'i> Debug for SourceTree<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cst_printer = CstPrinter::new(f);
        self.tree.root().walk(&mut cst_printer, self.tree)
    }
}

pub trait AstNode: Sized {
    fn cast(node: SyntaxNode, tree: &SyntaxTree) -> Option<Self>;

    fn range(&self, tree: &SyntaxTree) -> TextRange;

    fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str;
}

pub trait AstToken: Sized {
    fn cast(token: SyntaxToken, tree: &SyntaxTree) -> Option<Self>;

    fn range(&self, tree: &SyntaxTree) -> TextRange;

    fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str;
}

macro_rules! impl_ast_node {
    ($enum_name:ident::$variant:ident) => {
        pub struct $variant(SyntaxNode);

        impl AstNode for $variant {
            fn cast(node: SyntaxNode, tree: &SyntaxTree) -> Option<Self> {
                if node.kind(tree) == $enum_name::$variant {
                    Some(Self(node))
                } else {
                    None
                }
            }

            fn range(&self, tree: &SyntaxTree) -> TextRange {
                self.0.range(tree)
            }

            fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str {
                self.0.text(tree)
            }
        }
    };
}

macro_rules! impl_ast_token {
    ($enum_name:ident::$variant:ident) => {
        pub struct $variant(SyntaxToken);

        impl AstToken for $variant {
            fn cast(token: SyntaxToken, tree: &SyntaxTree) -> Option<Self> {
                if token.kind(tree) == $enum_name::$variant {
                    Some(Self(token))
                } else {
                    None
                }
            }

            fn range(&self, tree: &SyntaxTree) -> TextRange {
                self.0.range(tree)
            }

            fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str {
                self.0.text(tree)
            }
        }
    };
}

fn child_nodes<'t, A: AstNode>(
    node: &SyntaxNode,
    tree: &'t SyntaxTree,
) -> impl Iterator<Item = A> + 't {
    node.child_nodes(tree).filter_map(|n| A::cast(n, tree))
}

fn find_child_node<A: AstNode>(node: &SyntaxNode, tree: &SyntaxTree) -> Option<A> {
    child_nodes(node, tree).next()
}

fn child_tokens<'t, A: AstToken>(
    node: &SyntaxNode,
    tree: &'t SyntaxTree,
) -> impl Iterator<Item = A> + 't {
    node.child_tokens(tree).filter_map(|t| A::cast(t, tree))
}

fn find_child_token<A: AstToken>(node: &SyntaxNode, tree: &SyntaxTree) -> Option<A> {
    child_tokens(node, tree).next()
}

impl_ast_node!(NodeKind::Workout);

impl Workout {
    pub fn set_groups<'t>(&self, tree: &'t SyntaxTree) -> impl Iterator<Item = SetGroup> + 't {
        child_nodes(&self.0, tree)
    }
}

//     pub fn walk<W: TreeWalker>(&self, walker: &mut W, source: &str) -> Result<(), W::Err> {
//         self.0.walk(walker, source)
//     }
// }

impl_ast_node!(NodeKind::SetGroup);

impl SetGroup {
    pub fn exercise(&self, tree: &SyntaxTree) -> Option<Exercise> {
        find_child_node(&self.0, tree)
    }

    pub fn sets<'t>(&self, tree: &'t SyntaxTree) -> impl Iterator<Item = Set> + 't {
        child_nodes(&self.0, tree)
    }
}

impl_ast_node!(NodeKind::Exercise);

impl Exercise {
    pub fn ident(&self, tree: &SyntaxTree) -> Option<Ident> {
        find_child_token(&self.0, tree)
    }
}

impl_ast_node!(NodeKind::Set);

impl Set {
    pub fn weight(&self, tree: &SyntaxTree) -> Option<Weight> {
        find_child_node(&self.0, tree)
    }

    pub fn quantity(&self, tree: &SyntaxTree) -> Option<Quantity> {
        find_child_node(&self.0, tree)
    }
}

impl_ast_node!(NodeKind::Weight);

impl Weight {
    pub fn weight(&self, tree: &SyntaxTree) -> Option<WeightLiteral> {
        find_child_token(&self.0, tree)
    }

    pub fn bodyweight(&self, tree: &SyntaxTree) -> Option<Bodyweight> {
        find_child_token(&self.0, tree)
    }
}

pub enum WeightLiteral {
    Float(Float),
    Integer(Integer),
}

impl WeightLiteral {
    pub fn parse(&self, tree: &SyntaxTree) -> f64 {
        match self {
            WeightLiteral::Float(float) => float.parse(tree),
            WeightLiteral::Integer(int) => int.parse(tree) as f64,
        }
    }
}

impl AstToken for WeightLiteral {
    fn cast(token: SyntaxToken, tree: &SyntaxTree) -> Option<Self> {
        match token.kind(tree) {
            TokenKind::Float => Some(Self::Float(Float(token))),
            TokenKind::Integer => Some(Self::Integer(Integer(token))),
            _ => None,
        }
    }

    fn range(&self, tree: &SyntaxTree) -> TextRange {
        match self {
            WeightLiteral::Float(float) => float.range(tree),
            WeightLiteral::Integer(integer) => integer.range(tree),
        }
    }

    fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str {
        match self {
            WeightLiteral::Float(float) => float.text(tree),
            WeightLiteral::Integer(integer) => integer.text(tree),
        }
    }
}

pub enum Quantity {
    Reps(Reps),
    SimpleDuration(SimpleDuration),
    LongDuration(LongDuration),
}

impl AstNode for Quantity {
    fn cast(node: SyntaxNode, tree: &SyntaxTree) -> Option<Self> {
        match node.kind(tree) {
            NodeKind::Reps => Some(Self::Reps(Reps(node))),
            NodeKind::SimpleDuration => Some(Self::SimpleDuration(SimpleDuration(node))),
            NodeKind::LongDuration => Some(Self::LongDuration(LongDuration(node))),
            _ => None,
        }
    }

    fn range(&self, tree: &SyntaxTree) -> TextRange {
        match self {
            Quantity::Reps(reps) => reps.range(tree),
            Quantity::SimpleDuration(simple) => simple.range(tree),
            Quantity::LongDuration(long) => long.range(tree),
        }
    }

    fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str {
        match self {
            Quantity::Reps(reps) => reps.text(tree),
            Quantity::SimpleDuration(simple) => simple.text(tree),
            Quantity::LongDuration(long) => long.text(tree),
        }
    }
}

impl_ast_node!(NodeKind::Reps);

impl Reps {
    pub fn amount(&self, tree: &SyntaxTree) -> Option<Integer> {
        find_child_token(&self.0, tree)
    }
}

impl_ast_node!(NodeKind::SimpleDuration);

impl SimpleDuration {
    pub fn duration(&self, tree: &SyntaxTree) -> Option<Integer> {
        find_child_token(&self.0, tree)
    }

    pub fn unit(&self, tree: &SyntaxTree) -> Option<TimeUnit> {
        find_child_token(&self.0, tree)
    }
}

pub enum TimeUnit {
    Hour(Hour),
    Minute(Minute),
    Second(Second),
}

impl AstToken for TimeUnit {
    fn cast(token: SyntaxToken, tree: &SyntaxTree) -> Option<Self> {
        match token.kind(tree) {
            TokenKind::Hour => Some(Self::Hour(Hour(token))),
            TokenKind::Minute => Some(Self::Minute(Minute(token))),
            TokenKind::Second => Some(Self::Second(Second(token))),
            _ => None,
        }
    }

    fn range(&self, tree: &SyntaxTree) -> TextRange {
        match self {
            TimeUnit::Hour(hour) => hour.range(tree),
            TimeUnit::Minute(minute) => minute.range(tree),
            TimeUnit::Second(second) => second.range(tree),
        }
    }

    fn text<'t>(&self, tree: &'t SyntaxTree) -> &'t str {
        match self {
            TimeUnit::Hour(hour) => hour.text(tree),
            TimeUnit::Minute(minute) => minute.text(tree),
            TimeUnit::Second(second) => second.text(tree),
        }
    }
}

impl_ast_token!(TokenKind::Hour);
impl_ast_token!(TokenKind::Minute);
impl_ast_token!(TokenKind::Second);

impl_ast_node!(NodeKind::LongDuration);

impl LongDuration {
    fn has_hour_comp(&self) -> bool {
        // self.0.children.len() >= 4
        todo!()
    }

    pub fn hour(&self, tree: &SyntaxTree) -> Option<Integer> {
        if self.has_hour_comp() {
            find_child_token(&self.0, tree)
        } else {
            None
        }
    }

    pub fn minute(&self, tree: &SyntaxTree) -> Option<Integer> {
        if self.has_hour_comp() {
            child_tokens(&self.0, tree).nth(1)
        } else {
            find_child_token(&self.0, tree)
        }
    }

    pub fn second(&self, tree: &SyntaxTree) -> Option<Integer> {
        if self.has_hour_comp() {
            child_tokens(&self.0, tree).nth(2)
        } else {
            child_tokens(&self.0, tree).nth(1)
        }
    }
}

impl_ast_token!(TokenKind::Bodyweight);
impl_ast_token!(TokenKind::Ident);

impl_ast_token!(TokenKind::Float);

impl Float {
    pub fn parse(&self, tree: &SyntaxTree) -> f64 {
        self.text(tree).parse().unwrap()
    }
}

impl_ast_token!(TokenKind::Integer);

impl Integer {
    pub fn parse(&self, tree: &SyntaxTree) -> usize {
        self.text(tree).parse().unwrap()
    }
}
