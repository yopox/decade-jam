use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fight_status.pest"]
struct StatusParser;

fn test_valid(parent_rule: Rule, input: &str) {
    let result = StatusParser::parse(parent_rule, input);
    assert!(result.is_ok());
}

fn test_equals(parent_rule: Rule, input: &str, pairs: Vec<(&str, Rule)>) {
    let result = StatusParser::parse(parent_rule, input)
        .expect("Couldn't parse rule");

    for p in result {
        for (q, (str, r)) in p.into_inner().zip(pairs) {
            assert_eq!(q.as_str(), str);
            assert_eq!(q.as_rule(), r);
        }
        break
    }
}

#[test]
fn test_action_log() {
    test_equals(
        Rule::action_log, "! [A] 0 -> ATK [B]",
        vec![
            ("[A]", Rule::character),
            ("0", Rule::rule),
            ("ATK [B]", Rule::action),
        ]
    );
}

#[test]
fn test_status_log() {
    test_equals(
        Rule::status_log, ": [A] HP 10 -> 9",
        vec![
            ("[A]", Rule::character),
            ("HP", Rule::stat),
            ("10", Rule::value),
            ("9", Rule::value),
        ]
    );
}

#[test]
fn test_reaction_log() {
    test_equals(
        Rule::reaction_log, "> [B] RELIC 10 -> WAIT",
        vec![
            ("[B]", Rule::character),
            ("RELIC 10", Rule::reaction_origin),
            ("WAIT", Rule::action),
        ]
    );
}

#[test]
fn test_header_log() {
    test_equals(
        Rule::header, "- TURN 1",
        vec![("1", Rule::value)]
    );
}

#[test]
fn complete_fight() {
    test_valid(
        Rule::fight_status,
        "- TURN 1
         ! [A] 0 -> ATK [B]
         : [B] HP 10 -> 5
         ! [B] 0 -> ATK [A]
         : [A] HP 10 -> 8
         - TURN 2
         ! [A] 1 -> DEF
         ! [B] 0 -> ATK [A]
         : [A] HP 8 -> 7
         - TURN 3
         ! [A] 0 -> ATK [B]
         : [B] HP 5 -> 0
         = WON"
    );
}
