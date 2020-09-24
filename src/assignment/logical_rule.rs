use evalexpr::*;
use regex::Regex;

use std::error::Error;

use crate::assignment::arithmetic_rule::SubstitutionToken;

pub trait LogicalRule: Send + Sync {
    /// Returns `Some(SubstitutionToken)` if logical rule result is `true`, `None` otherwise.
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken>;
}

pub type RuleFn = Box<dyn Fn(bool, bool, bool) -> bool + Send + Sync>;

/// Stores `RuleFn` function that used for calculation and corresponding `SubstitutionToken`.
///
/// # Examples
///
/// ```
/// let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, _, _| a));
/// let res = rule.apply(true, false, false);
/// assert_eq!(res, Some(SubstitutionToken::M));
/// let res = rule.apply(false, false, false);
/// assert_eq!(res, None);
/// ```
pub struct LogicalRuleFn {
    token: SubstitutionToken,
    rule_fn: RuleFn,
}

impl LogicalRuleFn {
    /// Builds `LogicalRuleFn`
    ///
    /// # Arguments
    /// * `rule_fn` `RuleFn` function.
    pub fn new(token: SubstitutionToken, rule_fn: RuleFn) -> Self {
        Self { token, rule_fn }
    }
}

impl LogicalRule for LogicalRuleFn {
    /// Returns `Some(SunstitutionToken)` if result of stored function is `true`,
    /// None otherwise.
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken> {
        if (self.rule_fn)(a, b, c) {
            Some(self.token.clone())
        } else {
            None
        }
    }
}

/// Stores rule in a `String` and corresponding `SubstitutionToken`.
///
/// Rule string can contain only A, B or C variables and !, &&, ||, ==, != operators.
///
/// # Examples
///
/// ```
/// let rule = LogicalRuleStr::new(SubstitutionToken::M, "A && B").unwrap();
/// let res = rule.apply(true, true, false);
/// assert_eq!(res, Some(SubstitutionToken::M));
/// let res = rule.apply(false, true, false);
/// assert_eq!(res, None);
/// ```
pub struct LogicalRuleStr {
    token: SubstitutionToken,
    rule_str: String,
}

impl LogicalRuleStr {
    /// Validates provided rule string and builds `LogicalRuleFn`.
    /// Returns `Ok(LogicalRuleStr)` if validation is successful,
    /// otherwise returns error with description.
    pub fn new(token: SubstitutionToken, rule_str: String) -> Result<Self, Box<dyn Error>> {
        LogicalRuleStr::validate(&rule_str)?;
        Ok(Self { token, rule_str })
    }

    /// Validates provided rule string.
    /// Returns error if it contains invalid variables or operators,
    /// or if it's not compilable by `evalexpr`,
    /// otherwise returns `Ok`.
    fn validate(rule_str: &String) -> Result<(), Box<dyn Error>> {
        let re = Regex::new(r"^([ABC ]|&&|==|!=|!|\|\|)+$").unwrap();
        if !re.is_match(&rule_str) {
            Err("Expression contains invalid variables or operators.")?
        }

        // Try to evaluate expression with some input to check if it's valid for `evalexpr`.
        let context = context_map! {
            "A" => true,
            "B" => true,
            "C" => true,
        }
        .unwrap();
        eval_boolean_with_context(&rule_str, &context)?;

        Ok(())
    }
}

impl LogicalRule for LogicalRuleStr {
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken> {
        let context = context_map! {
            "A" => a,
            "B" => b,
            "C" => c,
        }
        .unwrap();
        let res = eval_boolean_with_context(&self.rule_str, &context).unwrap();

        if res {
            Some(self.token.clone())
        } else {
            None
        }
    }
}

#[test]
fn test_new() {
    let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, _, _| a));

    assert_eq!(rule.token, SubstitutionToken::M, "Invalid token is set.");
    assert_eq!(
        (rule.rule_fn)(true, true, true),
        true,
        "Invalid rule_fn is set."
    );
    assert_eq!(
        (rule.rule_fn)(false, true, true),
        false,
        "Invalid rule_fn is set."
    );
}

#[test]
fn test_apply() {
    let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, _, _| a));

    assert_eq!(rule.apply(true, true, true), Some(SubstitutionToken::M));
    assert_eq!(rule.apply(false, true, true), None);
}

#[test]
fn test_validate() {
    assert!(LogicalRuleStr::validate(&"A".to_owned()).is_ok());
    assert!(LogicalRuleStr::validate(&"A && B || C".to_owned()).is_ok());
    assert!(LogicalRuleStr::validate(&"A && !B || C".to_owned()).is_ok());
    assert!(LogicalRuleStr::validate(&"A == B".to_owned()).is_ok());
    assert!(LogicalRuleStr::validate(&"A != B".to_owned()).is_ok());

    assert_eq!(
        LogicalRuleStr::validate(&"".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"A || D".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"A + B".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"A - B".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"A * B".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"A / B".to_owned())
            .unwrap_err()
            .to_string(),
        "Expression contains invalid variables or operators."
    );

    assert_eq!(
        LogicalRuleStr::validate(&"A&&&&B".to_owned())
            .unwrap_err()
            .to_string(),
        "An operator expected 2 arguments, but got 1."
    );
    assert_eq!(
        LogicalRuleStr::validate(&"&&A".to_owned())
            .unwrap_err()
            .to_string(),
        "An operator expected 2 arguments, but got 1."
    );
}

#[test]
fn test_apply_str() {
    let rule = LogicalRuleStr::new(SubstitutionToken::M, "A".to_owned()).unwrap();

    assert_eq!(rule.apply(true, true, true), Some(SubstitutionToken::M));
    assert_eq!(rule.apply(false, true, true), None);
}
