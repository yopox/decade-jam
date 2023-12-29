use std::str::FromStr;
use crate::logic_prelude::{Action, Condition, Gate, Rule, Rune, Stat, Target};

fn read_rule(rule: &str) -> Option<Rule> {
    let mut runes = rule.split(" ").collect::<Vec<&str>>();
    runes.reverse();

    let Some(Rune::Gate(gate)) = read_gate(&mut runes) else { return None; };
    let Some(Rune::Action(action)) = read_action(&mut runes) else { return None; };

    return Some(Rule { gate, action });
}

fn read_gate(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "ID" => {
            let Some(Rune::Condition(condition)) = read_condition(rule) else { return None; };
            Some(Rune::Gate(Gate::ID(condition)))
        }
        "NOT" => {
            let Some(Rune::Condition(condition)) = read_condition(rule) else { return None; };
            Some(Rune::Gate(Gate::NOT(condition)))
        }
        _ => None,
    }
}

fn read_condition(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    let Some(Rune::Number(n)) = read_number(rule) else { return None; };
    match rune {
        "EXT" => Some(Rune::Condition(Condition::EveryXTurn(n))),
        "T=" => Some(Rune::Condition(Condition::OnTurn(n))),
        "T>" => Some(Rune::Condition(Condition::FromTurnX(n))),
        _ => None
    }
}

fn read_action(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "ATK" => match read_target(rule) {
            Some(Rune::Target(t)) => Some(Rune::Action(Action::Attack(t))),
            _ => None,
        }
        "DEF" => Some(Rune::Action(Action::Defense)),
        "W" => Some(Rune::Action(Action::Wait)),
        _ => None,
    }
}

fn read_target(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "SLF" => Some(Rune::Target(Target::Them)),
        _ => {
            let Some(Rune::Stat(s)) = read_stat(rule) else { return None; };
            match rune {
                "AL+" => Some(Rune::Target(Target::AllyMost(s))),
                "AL-" => Some(Rune::Target(Target::AllyLess(s))),
                "FO+" => Some(Rune::Target(Target::FoeMost(s))),
                "FO-" => Some(Rune::Target(Target::FoeLess(s))),
                _ => None,
            }
        }
    }
}

fn read_stat(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "HP" => Some(Rune::Stat(Stat::Health)),
        "ATK" => Some(Rune::Stat(Stat::Attack)),
        "DEF" => Some(Rune::Stat(Stat::Defense)),
        "SPD" => Some(Rune::Stat(Stat::Speed)),
        "NAT" => Some(Rune::Stat(Stat::Nature)),
        "DEM" => Some(Rune::Stat(Stat::Demon)),
        _ => None,
    }
}

fn read_number(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match u8::from_str(rune) {
        Ok(n) => Some(Rune::Number(n)),
        _ => None,
    }
}

#[test]
fn test() {
    assert_eq!(read_rule("ID"), None);
    assert_eq!(read_rule("ID EXT 2 W"),
               Some(Rule {
                   gate: Gate::ID(Condition::EveryXTurn(2)),
                   action: Action::Wait,
               }));
}