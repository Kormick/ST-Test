#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SubstitutionToken {
    M,
    P,
    T,
}

pub trait ArithmeticRule {
    fn apply(&self, d: f64, e: i32, f: i32) -> f64;
}

pub struct ArithmeticRuleFn {
    rule_fn: Box<dyn Fn(f64, i32, i32) -> f64>,
}

impl ArithmeticRuleFn {
    pub fn new(rule_fn: Box<dyn Fn(f64, i32, i32) -> f64>) -> Self {
        Self { rule_fn }
    }
}

impl ArithmeticRule for ArithmeticRuleFn {
    fn apply(&self, d: f64, e: i32, f: i32) -> f64 {
        (self.rule_fn)(d, e, f)
    }
}

#[test]
fn test_new() {
    let rule = ArithmeticRule::new(Box::new(|_, _, _| 2.0));
    assert_eq!((rule.rule_fn)(0.0, 0, 0), 2.0, "Invalid rule_fn is set.");
}

#[test]
fn test_apply() {
    let rule = ArithmeticRule::new(Box::new(|_, _, _| 2.0));
    assert_eq!(rule.apply(0.0, 0, 0), 2.0);

    let rule = ArithmeticRule::new(Box::new(|_, _, _| 1.0 / 0.0 as f64));
    assert!(!rule.apply(0.0, 0, 0).is_normal());
}
