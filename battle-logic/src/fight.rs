use std::os::macos::raw::stat;

use crate::fighter;
use crate::runes;
use crate::runes::Stat;
use crate::predefined;

#[derive(Debug, PartialEq)]
pub enum State {
    Victory(fighter::Team),
    Draw,
}

pub struct Status {
    pub(crate) turn: u8,
    fighters: Vec<fighter::Fighter>,
}

pub const MAX_TURNS: u8 = 50;

pub fn fight(fighters: Vec<fighter::Fighter>) -> State {
    let mut status = Status { turn: 0, fighters };
    loop {
        match turn(&mut status) {
            Some(result) => return result,
            None => (),
        }
    }
}

fn turn(status: &mut Status) -> Option<State> {
    status.turn += 1;
    if status.turn > MAX_TURNS {
        return Some(State::Draw);
    }

    println!("Turn {}", status.turn);

    // Order fighters by speed
    status.fighters.sort_by_key(|fighter| fighter.get_stat(Stat::Speed));
    status.fighters.reverse();

    for fighter in &status.fighters {
        if !fighter.alive { continue; }

        // Rule for the turn
        let rule = fighter.get_rule(status);

        // Perform the action
        fighter.perform(rule.get_action(), &status);
    }

    println!();

    return None;
}

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let fighters = vec![f];
    let status = Status { turn: 0, fighters };

    assert_eq!(status.fighters.get(0).unwrap().get_rule(&status), &predefined::rules::DEFAULT);
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![predefined::rules::ATTACK_2]);
    let fighters = vec![f];
    let status = Status { turn: 2, fighters };

    let rule = status.fighters.get(0).unwrap().get_rule(&status);
    assert_ne!(rule, &predefined::rules::DEFAULT);
    assert_eq!(rule, &predefined::rules::ATTACK_2);
}

#[test]
fn max_turns() {
    let mut f1 = fighter::dummy_fighter();
    let mut f2 = fighter::dummy_foe();
    let fighters = vec![f1, f2];

    assert_eq!(fight(fighters), State::Draw);
}

#[test]
fn order_by_speed() {
    let mut f1 = fighter::dummy_foe();
    let mut f2 = fighter::dummy_fighter();
    let mut status = Status { turn: 0, fighters: vec![f1, f2] };

    assert_eq!(status.fighters.get(0).unwrap().get_stat(Stat::Speed), 5);
    assert_eq!(status.fighters.get(1).unwrap().get_stat(Stat::Speed), 10);
    turn(&mut status);
    assert_eq!(status.fighters.get(0).unwrap().get_stat(Stat::Speed), 10);
    assert_eq!(status.fighters.get(1).unwrap().get_stat(Stat::Speed), 5);
}