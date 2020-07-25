use crate::fighter;
use crate::runes;

enum State {
    Unfinished,
    Victory(fighter::Team),
    Draw
}

pub struct Status {
    pub(crate) turn: u8,
    fighters: std::vec::Vec<fighter::Fighter>
}

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let fighters = vec![f];
    let status = Status { turn: 0, fighters };
    match status.fighters.get(0) {
        Some(fighter) => assert_eq!(fighter.get_rule(&status), &runes::predefined::DEFAULT),
        None => panic!("dummy_fighter expected")
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
        },
        None => panic!("dummy_fighter expected")
    }
}