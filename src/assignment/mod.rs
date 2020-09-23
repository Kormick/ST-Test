//! Implementation of assignment's main logic.

pub mod arithmetic_rule;
pub mod logical_rule;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::assignment::{
    arithmetic_rule::{ArithmeticRule, ArithmeticRuleFn, ArithmeticRuleStr, SubstitutionToken},
    logical_rule::{LogicalRule, LogicalRuleFn, LogicalRuleStr},
};

/// Set of input arguments for calculation.
#[derive(Default, Serialize, Deserialize)]
pub struct InputSet {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: f64,
    pub e: i32,
    pub f: i32,
}

/// Main class for substitution calculation.
/// Contains set of `LogicalRule` and `ArithmeticRule`
/// and implements methods to work with them.
///
/// # Examples
///
/// ```
/// let assignment = Assignment::new();
/// let l_rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|_, _, _| true));
/// let a_rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 42.0));
/// assignment.add_logical_rule(Box::new(l_rule));
/// assignment.add_arithmetic_rule(SubstitutionToken::M, Box::new(a_rule));
/// let res = assignment.eval(InputSet::default());
/// assert_eq!(res, (SubStitutionToken::M, 42.0));
/// ```
pub struct Assignment {
    logical_rules: Vec<Box<dyn LogicalRule>>,
    arithmetic_rules: HashMap<SubstitutionToken, Box<dyn ArithmeticRule>>,
}

impl Assignment {
    /// Builds `Assignment`.
    pub fn new() -> Self {
        Self {
            logical_rules: Vec::new(),
            arithmetic_rules: HashMap::new(),
        }
    }

    /// Adds predefined rules for `Assignment` object.
    ///
    /// # Arguments
    /// * `base` - If true adds base set of rules.
    /// * `custom` - If true adds custom set of rules.
    pub fn with_rules(mut self, base: bool, custom: bool) -> Self {
        if base {
            Self::add_base_rules(&mut self);
        }
        if custom {
            Self::add_custom_rules(&mut self);
        }
        self
    }

    /// Adds `LogicalRule` to `Assignment`.
    pub fn add_logical_rule(&mut self, rule: Box<dyn LogicalRule>) {
        self.logical_rules.push(rule);
    }

    /// Creates `LogicalRule` from `Fn` and adds it to `Assignment`.
    pub fn add_logical_rule_from_fn(
        &mut self,
        token: SubstitutionToken,
        rule_fn: logical_rule::RuleFn,
    ) {
        let rule = LogicalRuleFn::new(token, rule_fn);
        self.add_logical_rule(Box::new(rule));
    }

    /// Creates `LogicalRule` from `String` and adds it to `Assignment`.
    pub fn add_logical_rule_from_str(
        &mut self,
        token: SubstitutionToken,
        rule_str: String,
    ) -> Result<(), Box<dyn Error>> {
        let rule = LogicalRuleStr::new(token, rule_str)?;
        self.add_logical_rule(Box::new(rule));
        Ok(())
    }

    /// Adds `ArithmeticRule` to `Assignment`.
    pub fn add_arithmetic_rule(&mut self, token: SubstitutionToken, rule: Box<dyn ArithmeticRule>) {
        self.arithmetic_rules.insert(token, rule);
    }

    /// Creates `ArithmeticRule` from `Fn` and adds it to `Assignment`.
    pub fn add_arithmetic_rule_from_fn(
        &mut self,
        token: SubstitutionToken,
        rule_fn: arithmetic_rule::RuleFn,
    ) {
        let rule = ArithmeticRuleFn::new(rule_fn);
        self.add_arithmetic_rule(token, Box::new(rule));
    }

    /// Creates `ArithmeticRule` from `String` and adds it to `Assignment`.
    pub fn add_arithmetic_rule_from_str(
        &mut self,
        token: SubstitutionToken,
        rule_str: String,
    ) -> Result<(), Box<dyn Error>> {
        let rule = ArithmeticRuleStr::new(rule_str)?;
        self.add_arithmetic_rule(token, Box::new(rule));
        Ok(())
    }

    /// Calculates result of substitution rules for given arguments.
    ///
    /// First, goes through all logical rules to get `SubstitutionToken` for arithmetical rules.
    /// If there are several suitable logical rules, result of the last rule will be taken.
    /// Returns `Error` if there is no suitable rule for given input.
    ///
    /// Then, calculates result of arithmetical rule for found `SubstitutionToken`.
    /// Returns `Error` if there is no rule for `SubstitutionToken`.
    ///
    /// Returns tuple of `SubstitutionToken` and arithmetical rule result as `f64`.
    pub fn eval(&self, args: InputSet) -> Result<(SubstitutionToken, f64), Box<dyn Error>> {
        let mut token = None;
        for r in &self.logical_rules {
            let t = r.apply(args.a, args.b, args.c);
            if t.is_some() {
                token = t;
            }
        }

        let token = token.ok_or("Failed to apply logical rule.")?;

        let rule = self
            .arithmetic_rules
            .get(&token)
            .ok_or("Failed to find arithmetic rule for token.")?;

        Ok((token, rule.apply(args.d, args.e, args.f)))
    }

    /// Adds set of predefined base rules to `Assignment`.
    fn add_base_rules(obj: &mut Assignment) {
        obj.add_logical_rule_from_fn(SubstitutionToken::M, Box::new(|a, b, c| a && b && !c));
        obj.add_logical_rule_from_fn(SubstitutionToken::P, Box::new(|a, b, c| a && b && c));
        obj.add_logical_rule_from_fn(SubstitutionToken::T, Box::new(|a, b, c| !a && b && c));

        obj.add_arithmetic_rule_from_fn(
            SubstitutionToken::M,
            Box::new(|d, e, _| d + (d * e as f64 / 10.0)),
        );
        obj.add_arithmetic_rule_from_fn(
            SubstitutionToken::P,
            Box::new(|d, e, f| d + (d * (e - f) as f64 / 25.5)),
        );
        obj.add_arithmetic_rule_from_fn(
            SubstitutionToken::T,
            Box::new(|d, _, f| d - (d * f as f64 / 30.0)),
        );
    }

    /// Adds set of predefined custom rules to `Assignment`.
    fn add_custom_rules(obj: &mut Assignment) {
        obj.add_logical_rule_from_fn(SubstitutionToken::T, Box::new(|a, b, c| a && b && !c));
        obj.add_logical_rule_from_fn(SubstitutionToken::M, Box::new(|a, b, c| a && !b && c));

        obj.add_arithmetic_rule_from_fn(
            SubstitutionToken::P,
            Box::new(|d, e, _| 2.0 * d + (d * e as f64 / 100.0)),
        );
        obj.add_arithmetic_rule_from_fn(
            SubstitutionToken::M,
            Box::new(|d, e, f| f as f64 + d + (d * e as f64 / 100.0)),
        );
    }
}

#[test]
fn test_new() {
    let assignment = Assignment::new();

    assert!(assignment.logical_rules.is_empty());
    assert!(assignment.arithmetic_rules.is_empty());
}

#[test]
fn test_with_rules() {
    let assignment = Assignment::new().with_rules(false, false);
    assert!(assignment.logical_rules.is_empty());
    assert!(assignment.arithmetic_rules.is_empty());

    let assignment = Assignment::new().with_rules(true, false);
    assert!(!assignment.logical_rules.is_empty());
    assert!(!assignment.arithmetic_rules.is_empty());

    let assignment = Assignment::new().with_rules(false, true);
    assert!(!assignment.logical_rules.is_empty());
    assert!(!assignment.arithmetic_rules.is_empty());

    let assignment = Assignment::new().with_rules(true, true);
    assert!(!assignment.logical_rules.is_empty());
    assert!(!assignment.arithmetic_rules.is_empty());
}

#[test]
fn test_add_logical_rule() {
    let mut assignment = Assignment::new();

    let rule0 = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, _, _| a));
    assignment.add_logical_rule(Box::new(rule0));

    assert_eq!(assignment.logical_rules.len(), 1);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[0].apply(true, true, true),
        Some(SubstitutionToken::M)
    );
    assert_eq!(assignment.logical_rules[0].apply(false, true, true), None);

    let rule1 = LogicalRuleStr::new(SubstitutionToken::T, "B".to_owned()).unwrap();
    assignment.add_logical_rule(Box::new(rule1));

    assert_eq!(assignment.logical_rules.len(), 2);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[1].apply(true, true, true),
        Some(SubstitutionToken::T)
    );
    assert_eq!(assignment.logical_rules[1].apply(true, false, true), None);
}

#[test]
fn test_add_logical_rule_from_fn() {
    let mut assignment = Assignment::new();

    assignment.add_logical_rule_from_fn(SubstitutionToken::M, Box::new(|a, _, _| a));
    assert_eq!(assignment.logical_rules.len(), 1);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[0].apply(true, true, true),
        Some(SubstitutionToken::M)
    );
    assert_eq!(assignment.logical_rules[0].apply(false, true, true), None);

    assignment.add_logical_rule_from_fn(SubstitutionToken::P, Box::new(|_, b, _| b));
    assert_eq!(assignment.logical_rules.len(), 2);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[1].apply(true, true, true),
        Some(SubstitutionToken::P)
    );
    assert_eq!(assignment.logical_rules[1].apply(true, false, true), None);
}

#[test]
fn test_add_logical_rule_from_str() {
    let mut assignment = Assignment::new();

    assignment
        .add_logical_rule_from_str(SubstitutionToken::M, "A".to_owned())
        .expect("Should not fail.");

    assert_eq!(assignment.logical_rules.len(), 1);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[0].apply(true, true, true),
        Some(SubstitutionToken::M)
    );
    assert_eq!(assignment.logical_rules[0].apply(false, true, true), None);

    assignment
        .add_logical_rule_from_str(SubstitutionToken::T, "B".to_owned())
        .expect("Should not fail.");
    assert_eq!(assignment.logical_rules.len(), 2);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[1].apply(true, true, true),
        Some(SubstitutionToken::T)
    );
    assert_eq!(assignment.logical_rules[1].apply(true, false, true), None);

    assignment
        .add_logical_rule_from_str(SubstitutionToken::P, "Z+X".to_owned())
        .expect_err("Should fail.");
    assert_eq!(assignment.logical_rules.len(), 2);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
}

#[test]
fn test_add_arithmetic_rule() {
    let mut assignment = Assignment::new();

    let rule0 = ArithmeticRuleFn::new(Box::new(|d, _, _| d));
    assignment.add_arithmetic_rule(SubstitutionToken::M, Box::new(rule0));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 1);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::M].apply(2.0, 0, 0),
        2.0
    );

    let rule1 = ArithmeticRuleStr::new("E".to_owned()).unwrap();
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule1));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 2, 0),
        2.0
    );

    let rule2 = ArithmeticRuleFn::new(Box::new(|_, _, f| f as f64));
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule2));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 0, 2),
        2.0
    );
}

#[test]
fn test_add_arithmetic_rule_from_fn() {
    let mut assignment = Assignment::new();

    assignment.add_arithmetic_rule_from_fn(SubstitutionToken::M, Box::new(|d, _, _| d));
    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 1);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::M].apply(2.0, 0, 0),
        2.0
    );

    assignment.add_arithmetic_rule_from_fn(SubstitutionToken::T, Box::new(|_, e, _| e as f64));
    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 2, 0),
        2.0
    );

    assignment.add_arithmetic_rule_from_fn(SubstitutionToken::T, Box::new(|_, _, f| f as f64));
    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 0, 2),
        2.0
    );
}

#[test]
fn test_add_arithmetic_rule_from_str() {
    let mut assignment = Assignment::new();

    assignment
        .add_arithmetic_rule_from_str(SubstitutionToken::M, "D".to_owned())
        .expect("Should not fail.");

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 1);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::M].apply(2.0, 0, 0),
        2.0
    );

    assignment
        .add_arithmetic_rule_from_str(SubstitutionToken::T, "E".to_owned())
        .expect("Should not fail.");
    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 2, 0),
        2.0
    );

    assignment
        .add_arithmetic_rule_from_str(SubstitutionToken::P, "Z&&X".to_owned())
        .expect_err("Should fail.");
    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
}

#[test]
#[should_panic(expected = "Failed to apply logical rule.")]
fn test_eval_empty_logical_rules() {
    let mut assignment = Assignment::new();

    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 0.0));
    assignment.add_arithmetic_rule(SubstitutionToken::M, Box::new(rule));
    assignment.eval(InputSet::default()).unwrap();
}

#[test]
#[should_panic(expected = "Failed to find arithmetic rule for token.")]
fn test_eval_empty_arithmetic_rules() {
    let mut assignment = Assignment::new();

    let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|_, _, _| true));
    assignment.add_logical_rule(Box::new(rule));
    assignment.eval(InputSet::default()).unwrap();
}

#[test]
fn test_eval() {
    let mut assignment = Assignment::new();

    let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, _, _| a));
    assignment.add_logical_rule(Box::new(rule));
    let rule = LogicalRuleFn::new(SubstitutionToken::T, Box::new(|_, b, _| b));
    assignment.add_logical_rule(Box::new(rule));
    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 2.0));
    assignment.add_arithmetic_rule(SubstitutionToken::M, Box::new(rule));
    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 3.0));
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule));

    let res = assignment
        .eval(InputSet {
            a: true,
            b: false,
            c: false,
            d: 0.0,
            e: 0,
            f: 0,
        })
        .expect("eval() should return Ok()");
    assert_eq!(res, (SubstitutionToken::M, 2.0));

    let res = assignment
        .eval(InputSet {
            a: false,
            b: true,
            c: false,
            d: 0.0,
            e: 0,
            f: 0,
        })
        .expect("eval() should return Ok()");
    assert_eq!(res, (SubstitutionToken::T, 3.0));

    // Override logical rule to substitute to another arithmetic rule.
    let rule = LogicalRuleFn::new(SubstitutionToken::T, Box::new(|a, _, _| a));
    assignment.add_logical_rule(Box::new(rule));

    let res = assignment
        .eval(InputSet {
            a: true,
            b: false,
            c: false,
            d: 0.0,
            e: 0,
            f: 0,
        })
        .expect("eval() should return Ok()");
    assert_eq!(res, (SubstitutionToken::T, 3.0));

    // Override arithmetic rule.
    let rule = ArithmeticRuleFn::new(Box::new(|_, _, _| 4.0));
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule));

    let res = assignment
        .eval(InputSet {
            a: true,
            b: false,
            c: false,
            d: 0.0,
            e: 0,
            f: 0,
        })
        .expect("eval() should return Ok()");
    assert_eq!(res, (SubstitutionToken::T, 4.0));

    // Override logical rule to no arithmetic rule.
    let rule = LogicalRuleFn::new(SubstitutionToken::P, Box::new(|a, _, _| a));
    assignment.add_logical_rule(Box::new(rule));

    let res = assignment.eval(InputSet {
        a: true,
        b: false,
        c: false,
        d: 0.0,
        e: 0,
        f: 0,
    });

    assert_eq!(
        res.unwrap_err().to_string(),
        "Failed to find arithmetic rule for token."
    );
}
