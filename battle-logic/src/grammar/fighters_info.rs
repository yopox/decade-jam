use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fighters_info.pest"]
struct InfoParser;

fn test_valid(parent_rule: Rule, input: &str) {
    let result = InfoParser::parse(parent_rule, input);
    assert!(result.is_ok());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use lazy_static::lazy_static;
    use pest_test::{default_test_dir, PestTester, TestError};

    use crate::grammar::fighters_info::{Rule, InfoParser};

    lazy_static! {
        static ref TESTER: PestTester<Rule, InfoParser> =
          PestTester::new(default_test_dir(), "txt", Rule::fighters_info, HashSet::new());
    }

    #[test]
    fn test_complete_fight() -> Result<(), TestError<Rule>> {
        (*TESTER).evaluate_strict("info1")
    }
}