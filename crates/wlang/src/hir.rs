use crate::ast::{self, AstToken};

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
    pub fn lower(ast: ast::Workout, source: &str) -> Self {
        let set_groups = ast
            .set_groups()
            .map(|sg| SetGroup::lower(sg, source))
            .collect();
        Self { set_groups }
    }
}

impl SetGroup {
    fn lower(ast: ast::SetGroup, source: &str) -> Self {
        let exercise = ast
            .exercise()
            .and_then(|e| e.ident().map(|i| i.text(source).to_string()));

        let sets = ast.sets().map(|s| Set::lower(s, source)).collect();

        Self { exercise, sets }
    }
}

impl Set {
    fn lower(ast: ast::Set, source: &str) -> Self {
        let weight = ast.weight().map(|w| Weight::lower(w, source));
        let quantity = ast.quantity().map(|q| Quantity::lower(q, source));

        Self { weight, quantity }
    }
}

impl Weight {
    fn lower(ast: ast::Weight, source: &str) -> Self {
        match (ast.weight(), ast.bodyweight()) {
            (Some(weight), Some(_bw)) => Self::Bodyweight(Some(weight.parse(source))),
            (Some(weight), None) => Self::Straight(weight.parse(source)),
            (None, Some(_bw)) => Self::Bodyweight(None),
            _ => Self::Error,
        }
    }
}

impl Quantity {
    fn lower(ast: ast::Quantity, source: &str) -> Self {
        match ast {
            ast::Quantity::Reps(reps) => Self::lower_reps(reps, source),
            ast::Quantity::SimpleDuration(simple) => Self::lower_simple_duration(simple, source),
            ast::Quantity::LongDuration(long) => Self::lower_long_duration(long, source),
        }
    }

    fn lower_reps(reps: ast::Reps, source: &str) -> Self {
        if let Some(amount) = reps.amount().map(|i| i.parse(source)) {
            Self::Reps(amount)
        } else {
            Self::Error
        }
    }

    fn lower_simple_duration(simple: ast::SimpleDuration, source: &str) -> Self {
        let Some(duration) = simple.duration().map(|i| i.parse(source)) else { return Self::Error };

        let multiplier = simple
            .unit()
            .map(|unit| match unit {
                ast::TimeUnit::Hour(_) => 3600,
                ast::TimeUnit::Minute(_) => 60,
                ast::TimeUnit::Second(_) => 1,
            })
            .unwrap_or(1);

        Self::Duration(duration * multiplier)
    }

    fn lower_long_duration(long: ast::LongDuration, source: &str) -> Self {
        let hour = long.hour().map(|h| h.parse(source)).unwrap_or(0);
        let minute = long.minute().map(|m| m.parse(source)).unwrap_or(0);
        let second = long.second().map(|s| s.parse(source)).unwrap_or(0);

        let duration = (hour * 3600) + (minute * 60) + second;
        Self::Duration(duration)
    }
}
