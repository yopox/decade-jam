use crate::fighter;
use crate::runes;
use std::os::macos::raw::stat;

#[derive(Debug, PartialEq)]
enum State {
    Victory(fighter::Team),
    Draw,
}

pub struct Status {
    pub(crate) turn: u8,
    fighters: std::vec::Vec<fighter::Fighter>,
}

pub const MAX_TURNS: u8 = 50;

fn fight(fighters: Vec<fighter::Fighter>) -> State {
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
    if status.turn >= MAX_TURNS {
        return Some(State::Draw);
    }

    // Order fighters by speed
    status.fighters.sort_by_key(|fighter| -fighter.stats.speed);

    return None;
}

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let fighters = vec![f];
    let status = Status { turn: 0, fighters };

    assert_eq!(status.fighters.get(0).unwrap().get_rule(&status), &runes::predefined::DEFAULT);
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![runes::predefined::ATTACK_2]);
    let fighters = vec![f];
    let status = Status { turn: 2, fighters };

    let rule = status.fighters.get(0).unwrap().get_rule(&status);
    assert_ne!(rule, &runes::predefined::DEFAULT);
    assert_eq!(rule, &runes::predefined::ATTACK_2);
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
    let mut f1 = fighter::dummy_fighter();
    f1.stats.speed = 10;
    let mut f2 = fighter::dummy_foe();
    f2.stats.speed = 20;
    let mut status = Status { turn: 0, fighters: vec![f1, f2] };

    assert_eq!(status.fighters.get(0).unwrap().stats.speed, 10);
    assert_eq!(status.fighters.get(1).unwrap().stats.speed, 20);
    turn(&mut status);
    assert_eq!(status.fighters.get(0).unwrap().stats.speed, 20);
    assert_eq!(status.fighters.get(1).unwrap().stats.speed, 10);
}