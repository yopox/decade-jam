use std::str::FromStr;
use crate::logic_prelude::{Action, Condition, Gate, Rule, Stat, Target};

pub enum Rune {
    Gate(Gate),
    Condition(Condition),
    Action(Action),
    Target(Target),
    Stat(Stat),
    Number(u8),
}

fn read_rule(rule: &str) -> Option<Rule> {
    let mut runes = rule.split(" ").collect::<Vec<&str>>();
    runes.reverse();

    let Some(Rune::Gate(gate)) = read_gate(&mut runes) else { return None; };
    let Some(Rune::Action(action)) = read_action(&mut runes) else { return None; };

    if !runes.is_empty() { return None }

    return Some(Rule { gate, action });
}

fn read_gate(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "ID" | "NOT" => {
            let Some(Rune::Condition(condition)) = read_condition(rule) else { return None; };
            match rune {
                "ID" => Some(Rune::Gate(Gate::ID(condition))),
                "NOT" => Some(Rune::Gate(Gate::NOT(condition))),
                _ => None,
            }
        }
        _ => {
            let Some(Rune::Condition(c1)) = read_condition(rule) else { return None; };
            let Some(Rune::Condition(c2)) = read_condition(rule) else { return None; };
            match rune {
                "AND" => Some(Rune::Gate(Gate::AND(c1, c2))),
                "NAND" => Some(Rune::Gate(Gate::NAND(c1, c2))),
                "OR" => Some(Rune::Gate(Gate::OR(c1, c2))),
                "XOR" => Some(Rune::Gate(Gate::XOR(c1, c2))),
                "NOR" => Some(Rune::Gate(Gate::NOR(c1, c2))),
                "NXOR" => Some(Rune::Gate(Gate::NXOR(c1, c2))),
                _ => None,
            }
        }
    }
}

fn read_condition(rule: &mut Vec<&str>) -> Option<Rune> {
    let Some(rune) = rule.pop() else { return None; };
    match rune {
        "EXT" | "T=" | "T>" => {
            let Some(Rune::Number(n)) = read_number(rule) else { return None; };
            match rune {
                "EXT" => Some(Rune::Condition(Condition::EveryXTurn(n))),
                "T=" => Some(Rune::Condition(Condition::OnTurn(n))),
                "T>" => Some(Rune::Condition(Condition::FromTurnX(n))),
                _ => None
            }
        }
        "HP>" | "HP<" => {
            let Some(Rune::Number(n)) = read_number(rule) else { return None; };
            let Some(Rune::Target(t)) = read_target(rule) else { return None; };
            match rune {
                "HP>" => Some(Rune::Condition(Condition::MoreXHP(n, t))),
                "HP<" => Some(Rune::Condition(Condition::LessXHP(n, t))),
                _ => None
            }
        }
        "STA" => {
            // TODO
            None
        }
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
    let c1 = Condition::EveryXTurn(2);
    let c2 = Condition::EveryXTurn(3);
    for (id, gate) in [
        ("ID", Gate::ID(c1.clone())),
        ("NOT", Gate::NOT(c1.clone()))
    ] {
        assert_eq!(read_rule(id), None);
        assert_eq!(read_rule(&format!("{} EXT 2 W", id)),
                   Some(Rule { gate, action: Action::Wait, }));
    }

    for (id, gate) in [
        ("AND", Gate::AND(c1.clone(), c2.clone())),
        ("NAND", Gate::NAND(c1.clone(), c2.clone())),
        ("OR", Gate::OR(c1.clone(), c2.clone())),
        ("XOR", Gate::XOR(c1.clone(), c2.clone())),
        ("NOR", Gate::NOR(c1.clone(), c2.clone())),
        ("NXOR", Gate::NXOR(c1.clone(), c2.clone())),
    ] {
        assert_eq!(read_rule(id), None);
        assert_eq!(read_rule(&format!("{} EXT 2 EXT 3 W", id)),
                   Some(Rule { gate, action: Action::Wait, }));
    }

    for (id, cond) in [
        ("EXT 2", Condition::EveryXTurn(2)),
        ("T= 2", Condition::OnTurn(2)),
        ("T> 2", Condition::FromTurnX(2)),
        ("HP> 10 SLF", Condition::MoreXHP(10, Target::Them)),
        ("HP< 10 SLF", Condition::LessXHP(10, Target::Them)),
    ] {
        assert_eq!(read_rule(&format!("ID {}", id)), None);
        assert_eq!(read_rule(&format!("ID {} W", id)),
                   Some(Rule { gate: Gate::ID(cond), action: Action::Wait }));
    }

    for (id, action) in [
        ("ATK SLF", Action::Attack(Target::Them)),
        ("DEF", Action::Defense),
        ("W", Action::Wait),
    ] {
        assert_eq!(read_rule(&format!("ID EXT 1 {}", id)),
                   Some(Rule { gate: Gate::ID(Condition::EveryXTurn(1)), action }))
    }
}