use evalexpr::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

use std::error::Error;

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

pub type RuleFn = Box<dyn Fn(f64, i32, i32) -> f64 + Send + Sync>;

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

/// Stores rule in a `String` that used for calculation.
///
/// Rule can contain only D, E or F variables and arithmetical operators.
///
/// # Examples
///
/// ```
/// let rule = ArithmeticRuleStr::from_str("D + E");
/// let res = rule.apply(1.0, 2, 0);
/// assert_eq!(res, 3.0);
/// ```
pub struct ArithmeticRuleStr {
    rule_str: String,
}

impl ArithmeticRuleStr {
    pub fn new(rule_str: String) -> Result<Self, Box<dyn Error>> {
        ArithmeticRuleStr::validate(&rule_str)?;
        Ok(Self { rule_str })
    }

    /// Validates provided rule string.
    /// Returns error if it contains invalid variables or operators,
    /// or if it's not compilable by `evalexpr`,
    /// otherwise returns `Ok`.
    fn validate(rule_str: &String) -> Result<(), Box<dyn Error>> {
        let re = Regex::new(r"^([\dDEF ]|\+|-|\*|/|\(|\))+$").unwrap();
        if !re.is_match(&rule_str) {
            Err("Expression contains invalid variables or operators.")?
        }

        // Try to evaluate expression with some input to check if it's valid for `evalexpr`.
        let context = context_map! {
            "D" => 0.0,
            "E" => 0 as f64,
            "F" => 0 as f64,
        }
        .unwrap();
        eval_float_with_context(&rule_str, &context)?;

        Ok(())
    }
}

impl ArithmeticRule for ArithmeticRuleStr {
    fn apply(&self, d: f64, e: i32, f: i32) -> f64 {
        let context = context_map! {
            "D" => d,
            "E" => e as f64,
            "F" => f as f64,
        }
        .unwrap();

        eval_float_with_context(&self.rule_str, &context).unwrap()
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

#[test]
fn test_validate() {
    assert!(ArithmeticRuleStr::validate(&"D".to_owned()).is_ok());
    assert!(ArithmeticRuleStr::validate(&"-D + E".to_owned()).is_ok());
    assert!(ArithmeticRuleStr::validate(&"D * (-E + F)".to_owned()).is_ok());
    assert!(ArithmeticRuleStr::validate(&"-2 * D".to_owned()).is_ok());

    assert_eq!(
        ArithmeticRuleStr::validate(&"".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"A".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"D && E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"D || E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"D == E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"D != E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );

    assert_eq!(
        ArithmeticRuleStr::validate(&"/D * E".to_owned())
            .unwrap_err()
            .to_string(),
        "An operator expected 2 arguments, but got 1."
    );
    assert_eq!(
        ArithmeticRuleStr::validate(&"D ** E".to_owned())
            .unwrap_err()
            .to_string(),
        "An operator expected 2 arguments, but got 1."
    );
}

#[test]
fn test_apply_str() {
    let rule = ArithmeticRuleStr::new("D".to_owned()).unwrap();
    assert_eq!(rule.apply(0.0, 0, 0), 0.0);
    assert_eq!(rule.apply(1.0, 0, 0), 1.0);

    let rule = ArithmeticRuleStr::new("2 * D * E + F".to_owned()).unwrap();
    assert_eq!(rule.apply(2.0, 3, 4), 16.0);

    let rule = ArithmeticRuleStr::new("D / 0".to_owned()).unwrap();
    assert!(!rule.apply(1.0, 0, 0).is_normal());
}
