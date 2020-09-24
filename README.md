# ST-Test

This is a test task.

## Substitution algorithm
First, expressions are split to 3 types:
* Logical expressions - logical rules that result in substitution token for arithmetic rule.
* Arithmetical expressions - arithmetic rules that result in floating point number.
* Substitution tokens - tokens that define which arithmetic rule to substitute from logical rule.

For each given input we go through all logic rules to get substitution token for arithmetic rule. If there are several rules that can be applied, result of the last rule will be taken. If there is no rule that can be applied for this input, returns error.
Then, we calculate result of arithmetic rule for acquired token. If there is no arithmetic rule for this token, returns error.
Acquired token and calculation result provided as output.

## Implementation
Implementation consist of two main modules: `assignment` and `actix_app`.

### mod `assignment`
#### struct `Assignment`
Handles substitution rules and provides interface to work with them: adding/deleting rules, evaluating result for given input.

Stores logical rules in `Vec` and arithmetic rules in `HashMap` with `SubstitutionToken` key. 
These containers are selected because there can be several logical rules that point to one substitution, so we need to go though all logical rules available,
and only one arithmetic rule can be substituted from token, so we are not interested in keeping the rule if it gets overridden by another.

Methods `add_*_rule`, `add_*_rule_from_fn` and `add_*_rule_from_str` provide interface to add new rule object directly or to build it and add from `Fn` or `String` accordingly.

Method `remove_rules` provides interface to remove all rules from `Assignment`.

Method `eval` calculates result for current substitution rules.

Also, implements methods `add_base_rules` and `add_custom_rules` to add predefined rules from task description to `Assignment`.

#### trait `LogicalRule`
Provides `apply` method interface that takes 3 `bool` values and returns substitution token for arithmetic rule.

There are 2 derived implementations for `LogicalRule`:
* `LogicalRuleFn` - handles logical substitution rule as `Fn` with `(bool, bool, bool) -> bool` signature (e.g., `|a, b, c| a && b && c`).
* `LogicalRuleStr` - handles logical substitution rule as `String`, which is evaluated with `evalexpr` library (e.g., `"A && B && C"`).
    Rule string can contain only A, B or C variables and !, &&, ||, ==, != operators.
    This approach should be more human-friendly.

#### trait `ArithmeticRule`
Provides `apply` method interface that takes `f64`, `i32` and `i32` values and returns result of expression evaluation as `f64`.

There are 2 derived implementations for `ArithmeticRule`:
* `ArithmeticRuleFn` - handles arithmetic substitution as `Fn` with `(f64, i32, i32) -> f64` signature (e.g., `|d, e, f| d + e * f`).
* `ArithmeticRuleStr` - handles arithmetic substitution as `String`, which is evaluated with `evalexpr` library (e.g., `D + E * F`).
    Rule string can contain only D, E, or F variables and +, -, *, \/ operators.
    This approach should be more human-friendly.

### mod actix_app
Simple actix server application that provides REST API for assignment.

Implements several endpoints:
* `/add_logical_rule`
    Adds new logical rule to `Assignment`.
    Rule should be provided as JSON:
    ```
    {
        "token": "M",
        "rule_str": "A && B"
    }
    ```
    Returns OK if rule added successfully.
    Returns BAD_REQUEST with error message otherwise.

* `/add_arithmetic_rule`
    Adds new arithmetic rule to `Assignment`.
    Rule should be provided as JSON:
    ```
    {
        "token": "M",
        "rule_str": "D + E"
    }
    ```
    Returns OK if rule added successfully.
    Returns BAD_REQUEST with error message otherwise.

* `/remove_rules`
    Removes rules from `Assignment`

* `/eval`
    Calculates result for current substitution rules for given input.
    Input should be provided as JSON:
    ```
    {
        "a": true,
        "b": false,
        "c": true,
        "d": 1.2,
        "e": 3,
        "f": 4
    }
    ```
    Returns OK with tuple of token and calculation result as JSON.
    Returns BAD_REQUEST with error message otherwise.
