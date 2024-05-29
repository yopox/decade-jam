use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fight_status.pest"]
struct StatusParser;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use lazy_static::lazy_static;
    use pest_test::{default_test_dir, PestTester, TestError};

    use crate::grammar::fight_status::{Rule, StatusParser};

    lazy_static! {
        static ref TESTER: PestTester<Rule, StatusParser> =
          PestTester::new(default_test_dir(), "txt", Rule::fight_status, HashSet::new());
    }

    #[test]
    fn test_complete_fight() -> Result<(), TestError<Rule>> {
        (*TESTER).evaluate_strict("complete_fight")
    }
}