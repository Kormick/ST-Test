use crate::assignment::arithmetic_rule::SubstitutionToken;

pub trait LogicalRule {
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken>;
}

pub struct LogicalRuleFn {
    token: SubstitutionToken,
    rule_fn: Box<dyn Fn(bool, bool, bool) -> bool>,
}

impl LogicalRuleFn {
    pub fn new(token: SubstitutionToken, rule_fn: Box<dyn Fn(bool, bool, bool) -> bool>) -> Self {
        Self { token, rule_fn }
    }
}

impl LogicalRule for LogicalRuleFn {
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken> {
        if (self.rule_fn)(a, b, c) {
            Some(self.token.clone())
        } else {
            None
        }
    }
}

#[test]
fn test_new() {
    let rule = LogicalRule::new(SubstitutionToken::M, Box::new(|a, _, _| a));

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
    let rule = LogicalRule::new(SubstitutionToken::M, Box::new(|a, _, _| a));

    assert_eq!(rule.apply(true, true, true), Some(SubstitutionToken::M));
    assert_eq!(rule.apply(false, true, true), None);
}
