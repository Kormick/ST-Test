use crate::assignment::arithmetic_rule::SubstitutionToken;

pub trait LogicalRule: Send + Sync {
    /// Returns `Some(SubstitutionToken)` if logical rule result is `true`, `None` otherwise.
    fn apply(&self, a: bool, b: bool, c: bool) -> Option<SubstitutionToken>;
}

type RuleFn = Box<dyn Fn(bool, bool, bool) -> bool + Send + Sync>;

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
