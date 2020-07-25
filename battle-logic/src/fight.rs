use crate::fighter;
use crate::runes;

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
    return None;
}

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let fighters = vec![f];
    let status = Status { turn: 0, fighters };
    match status.fighters.get(0) {
        Some(fighter) => assert_eq!(fighter.get_rule(&status), &runes::predefined::DEFAULT),
        None => panic!("dummy_fighter expected"),
    }
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![runes::predefined::ATTACK_2]);
    let fighters = vec![f];
    let status = Status { turn: 2, fighters };

    match status.fighters.get(0) {
        Some(fighter) => {
            let rule = fighter.get_rule(&status);
            assert_ne!(rule, &runes::predefined::DEFAULT);
            assert_eq!(rule, &runes::predefined::ATTACK_2);
        }
        None => panic!("dummy_fighter expected"),
    }
}

#[test]
fn max_turns() {
    let mut f1 = fighter::dummy_fighter();
    let mut f2 = fighter::dummy_foe();
    let fighters = vec![f1, f2];
    assert_eq!(fight(fighters), State::Draw);
}
