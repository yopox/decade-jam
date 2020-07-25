use crate::fighter;
use crate::fighter::Fighter;

enum State {
    Unfinished,
    Victory(fighter::Team),
    Draw
}

pub struct Status {
    pub(crate) turn: u8,
    fighters: std::vec::Vec<Fighter>
}

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let mut fighters = Vec::new();
    fighters.push(f);
    let status = Status { turn: 0, fighters };
    match status.fighters.get(0) {
        Some(fighter) => assert_eq!(fighter.get_rule(&status), &crate::runes::predefined::DEFAULT),
        None => panic!("dummy_fighter expected")
    }
}