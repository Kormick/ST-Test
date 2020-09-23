use eval::Expr;
use regex::{Captures, Regex};
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

/// Stores rule_in a `String` that used for calculation.
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
        let rule_str = ArithmeticRuleStr::validate(rule_str)?;
        Ok(Self { rule_str })
    }

    fn validate(rule_str: String) -> Result<String, Box<dyn Error>> {
        let re = Regex::new(r"^([\dDEF ]|\+|-|\*|/|\(|\))+$").unwrap();
        if !re.is_match(&rule_str) {
            Err("Expression contains invalid variables or operators.")?
        }

        // Remove all whitespaces.
        let rule_str: String = rule_str.chars().filter(|c| !c.is_whitespace()).collect();

        // Wrap all '-VAR' expressions with in '()', as it's required by eval::Expr.
        let re = Regex::new(r"-[\dDEF]").unwrap();
        let rule_str = re.replace_all(&rule_str, |caps: &Captures| format!("({})", &caps[0]));

        // Try to compile expression to check if it's valid.
        Expr::new(rule_str.clone()).compile()?;

        Ok(String::from(rule_str))
    }
}

impl ArithmeticRule for ArithmeticRuleStr {
    fn apply(&self, d: f64, e: i32, f: i32) -> f64 {
        let res = Expr::new(&self.rule_str)
            .value("D", d)
            .value("E", e)
            .value("F", f)
            .exec()
            .unwrap()
            .as_f64();

        if res.is_none() {
            // `eval::Expr` does not handle division by zero correctly :c
            f64::NAN
        } else {
            res.unwrap()
        }
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
    assert_eq!(
        ArithmeticRuleStr::validate("".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );

    assert_eq!(ArithmeticRuleStr::validate("D".to_owned()).unwrap(), "D");
    assert_eq!(
        ArithmeticRuleStr::validate("A".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );

    assert_eq!(
        ArithmeticRuleStr::validate("-D + E".to_owned()).unwrap(),
        "(-D)+E"
    );
    assert_eq!(
        ArithmeticRuleStr::validate("D * (-E + F)".to_owned()).unwrap(),
        "D*((-E)+F)"
    );
    assert_eq!(
        ArithmeticRuleStr::validate("2 * D".to_owned()).unwrap(),
        "2*D"
    );

    assert_eq!(
        ArithmeticRuleStr::validate("D && E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate("D || E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate("D == E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        ArithmeticRuleStr::validate("D != E".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );

    assert_eq!(
        ArithmeticRuleStr::validate("/D * E".to_owned())
            .unwrap_err()
            .to_string(),
        eval::Error::StartWithNonValueOperator.to_string()
    );
    assert_eq!(
        ArithmeticRuleStr::validate("D ** E".to_owned())
            .unwrap_err()
            .to_string(),
        eval::Error::DuplicateOperatorNode.to_string()
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
