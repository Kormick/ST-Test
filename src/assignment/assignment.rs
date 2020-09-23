use std::{collections::HashMap, error::Error};

use crate::assignment::{
    arithmetic_rule::{ArithmeticRule, ArithmeticRuleFn, SubstitutionToken},
    logical_rule::{LogicalRule, LogicalRuleFn},
};

#[derive(Default)]
struct InputSet {
    a: bool,
    b: bool,
    c: bool,
    d: f64,
    e: i32,
    f: i32,
}

struct Assignment {
    logical_rules: Vec<Box<dyn LogicalRule>>,
    arithmetic_rules: HashMap<SubstitutionToken, Box<dyn ArithmeticRule>>,
}

impl Assignment {
    fn new() -> Self {
        Self {
            logical_rules: Vec::new(),
            arithmetic_rules: HashMap::new(),
        }
    }

    fn add_logical_rule(&mut self, rule: Box<dyn LogicalRule>) {
        self.logical_rules.push(rule);
    }

    fn add_arithmetic_rule(&mut self, token: SubstitutionToken, rule: Box<dyn ArithmeticRule>) {
        self.arithmetic_rules.insert(token, rule);
    }

    fn eval(&self, args: InputSet) -> Result<(SubstitutionToken, f64), Box<dyn Error>> {
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
}

#[test]
fn test_new() {
    let assignment = Assignment::new();

    assert!(assignment.logical_rules.is_empty());
    assert!(assignment.arithmetic_rules.is_empty());
}

#[test]
fn test_add_logical_rule() {
    let mut assignment = Assignment::new();

    let rule0 = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, b, c| a));
    assignment.add_logical_rule(Box::new(rule0));

    assert_eq!(assignment.logical_rules.len(), 1);
    assert_eq!(assignment.arithmetic_rules.len(), 0);
    assert_eq!(
        assignment.logical_rules[0].apply(true, true, true),
        Some(SubstitutionToken::M)
    );
    assert_eq!(assignment.logical_rules[0].apply(false, true, true), None);

    let rule1 = LogicalRuleFn::new(SubstitutionToken::T, Box::new(|a, b, c| b));
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
fn test_add_arithmetic_rule() {
    let mut assignment = Assignment::new();

    let rule0 = ArithmeticRuleFn::new(Box::new(|d, e, f| d));
    assignment.add_arithmetic_rule(SubstitutionToken::M, Box::new(rule0));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 1);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::M].apply(2.0, 0, 0),
        2.0
    );

    let rule1 = ArithmeticRuleFn::new(Box::new(|d, e, f| e as f64));
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule1));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 2, 0),
        2.0
    );

    let rule2 = ArithmeticRuleFn::new(Box::new(|d, e, f| f as f64));
    assignment.add_arithmetic_rule(SubstitutionToken::T, Box::new(rule2));

    assert_eq!(assignment.logical_rules.len(), 0);
    assert_eq!(assignment.arithmetic_rules.len(), 2);
    assert_eq!(
        assignment.arithmetic_rules[&SubstitutionToken::T].apply(0.0, 0, 2),
        2.0
    );
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

    let rule = LogicalRuleFn::new(SubstitutionToken::M, Box::new(|a, b, c| a));
    assignment.add_logical_rule(Box::new(rule));
    let rule = LogicalRuleFn::new(SubstitutionToken::T, Box::new(|a, b, c| b));
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
    let rule = LogicalRuleFn::new(SubstitutionToken::T, Box::new(|a, b, c| a));
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
    let rule = LogicalRuleFn::new(SubstitutionToken::P, Box::new(|a, b, c| a));
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
        res.unwrap_err().description(),
        "Failed to find arithmetic rule for token."
    );
}
