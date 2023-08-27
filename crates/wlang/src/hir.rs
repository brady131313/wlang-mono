use crate::ast::{self, AstToken, SyntaxTree};

#[derive(Debug)]
pub struct Workout {
    set_groups: Vec<SetGroup>,
}

#[derive(Debug)]
pub struct SetGroup {
    exercise: Option<String>,
    sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    weight: Option<Weight>,
    quantity: Option<Quantity>,
}

#[derive(Debug)]
pub enum Weight {
    Error,
    Straight(f64),
    Bodyweight(Option<f64>),
}

#[derive(Debug)]
pub enum Quantity {
    Error,
    Duration(usize),
    Reps(usize),
}

impl Workout {
    pub fn lower(ast: ast::Workout, tree: &SyntaxTree) -> Self {
        let set_groups = ast
            .set_groups(tree)
            .map(|sg| SetGroup::lower(sg, tree))
            .collect();
        Self { set_groups }
    }
}

impl SetGroup {
    fn lower(ast: ast::SetGroup, tree: &SyntaxTree) -> Self {
        let exercise = ast
            .exercise(tree)
            .and_then(|e| e.ident(tree).map(|i| i.text(tree).to_string()));

        let sets = ast.sets(tree).map(|s| Set::lower(s, tree)).collect();

        Self { exercise, sets }
    }
}

impl Set {
    fn lower(ast: ast::Set, tree: &SyntaxTree) -> Self {
        let weight = ast.weight(tree).map(|w| Weight::lower(w, tree));
        let quantity = ast.quantity(tree).map(|q| Quantity::lower(q, tree));

        Self { weight, quantity }
    }
}

impl Weight {
    fn lower(ast: ast::Weight, tree: &SyntaxTree) -> Self {
        match (ast.weight(tree), ast.bodyweight(tree)) {
            (Some(weight), Some(_bw)) => Self::Bodyweight(Some(weight.parse(tree))),
            (Some(weight), None) => Self::Straight(weight.parse(tree)),
            (None, Some(_bw)) => Self::Bodyweight(None),
            _ => Self::Error,
        }
    }
}

impl Quantity {
    fn lower(ast: ast::Quantity, tree: &SyntaxTree) -> Self {
        match ast {
            ast::Quantity::Reps(reps) => Self::lower_reps(reps, tree),
            ast::Quantity::SimpleDuration(simple) => Self::lower_simple_duration(simple, tree),
            ast::Quantity::LongDuration(long) => Self::lower_long_duration(long, tree),
        }
    }

    fn lower_reps(reps: ast::Reps, tree: &SyntaxTree) -> Self {
        if let Some(amount) = reps.amount(tree).map(|i| i.parse(tree)) {
            Self::Reps(amount)
        } else {
            Self::Error
        }
    }

    fn lower_simple_duration(simple: ast::SimpleDuration, tree: &SyntaxTree) -> Self {
        let Some(duration) = simple.duration(tree).map(|i| i.parse(tree)) else {
            return Self::Error;
        };

        let multiplier = simple
            .unit(tree)
            .map(|unit| match unit {
                ast::TimeUnit::Hour(_) => 3600,
                ast::TimeUnit::Minute(_) => 60,
                ast::TimeUnit::Second(_) => 1,
            })
            .unwrap_or(1);

        Self::Duration(duration * multiplier)
    }

    fn lower_long_duration(long: ast::LongDuration, tree: &SyntaxTree) -> Self {
        let hour = long.hour(tree).map(|h| h.parse(tree)).unwrap_or(0);
        let minute = long.minute(tree).map(|m| m.parse(tree)).unwrap_or(0);
        let second = long.second(tree).map(|s| s.parse(tree)).unwrap_or(0);

        let duration = (hour * 3600) + (minute * 60) + second;
        Self::Duration(duration)
    }
}
