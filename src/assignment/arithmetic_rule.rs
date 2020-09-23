use serde::{Deserialize, Serialize};

/// Contains possible substitution tokens for `LogicalRule` and `ArithmeticRule`.
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum SubstitutionToken {
    M,
    P,
    T,
}

pub trait ArithmeticRule: Send + Sync {
    /// Returns result of rule calculation as `f64`.
    fn apply(&self, d: f64, e: i32, f: i32) -> f64;
}

type RuleFn = Box<dyn Fn(f64, i32, i32) -> f64 + Send + Sync>;

/// Stores `RuleFn` function that used for for calculation.
///
/// # Examples
///
/// ```
/// let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 42.0));
/// let res = rule.apply(0.0, 0, 0);
/// assert_eq!(res, 42.0);
/// ```
pub struct ArithmeticRuleFn {
    rule_fn: RuleFn,
}

impl ArithmeticRuleFn {
    /// Builds `ArithmeticRuleFn`.
    ///
    /// # Arguments
    /// * `rule_fn` `RuleFn` function.
    pub fn new(rule_fn: RuleFn) -> Self {
        Self { rule_fn }
    }
}

impl ArithmeticRule for ArithmeticRuleFn {
    /// Returns result of stored function for given arguments.
    fn apply(&self, d: f64, e: i32, f: i32) -> f64 {
        (self.rule_fn)(d, e, f)
    }
}

#[test]
fn test_new() {
    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 2.0));
    assert_eq!((rule.rule_fn)(0.0, 0, 0), 2.0, "Invalid rule_fn is set.");
}

#[test]
fn test_apply() {
    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 2.0));
    assert_eq!(rule.apply(0.0, 0, 0), 2.0);

    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 1.0 / 0.0 as f64));
    assert!(!rule.apply(0.0, 0, 0).is_normal());
}
