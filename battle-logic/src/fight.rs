use std::iter::Filter;
use std::ops::DivAssign;
use std::os::macos::raw::stat;
use std::slice::Iter;

use crate::fighter;
use crate::fighter::Fighter;
use crate::predefined;
use crate::predefined::rules::AllRules;
use crate::predefined::spells::AllSpells;
use crate::predefined::weapons::AllWeapons;
use crate::runes;
use crate::runes::{Stat, Target};

#[derive(Debug, PartialEq)]
pub enum State {
    AlliesVictory,
    EnemiesVictory,
    Draw,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FighterID {
    Ally(usize),
    Enemy(usize),
    None,
}

impl FighterID {
    pub fn is_ally(&self) -> bool {
        match self {
            FighterID::Ally(_) => true,
            _ => false,
        }
    }
}

pub struct Fight {
    pub turn: u8,
    pub fighters: Vec<(FighterID, fighter::Fighter)>,
}

impl Fight {
    pub fn start(mut team1: Vec<Fighter>, mut team2: Vec<Fighter>) -> State {

        // Build fighters array
        let mut fighters = Vec::new();
        team1.into_iter().enumerate().for_each(|(i, f)| fighters.push(
            (FighterID::Ally(i), f)
        ));
        team2.into_iter().enumerate().for_each(|(i, f)| fighters.push(
            (FighterID::Enemy(i), f)
        ));

        let mut fight = Fight { turn: 0, fighters };

        loop {
            match fight.turn() {
                Some(result) => return result,
                None => (),
            }
        }
    }

    pub fn get_fighter(&self, id: &FighterID) -> &Fighter {
        match self.fighters.iter().find(|(fID, _)| *fID == *id) {
            Some((_, x)) => x,
            None => panic!("Tried to get &Fighter from wrong FighterID."),
        }
    }

    pub fn turn(&mut self) -> Option<State> {
        self.turn += 1;
        if self.turn > MAX_TURNS {
            return Some(State::Draw);
        }

        println!("Turn {}", self.turn);

        // Order fighters by speed
        self.fighters.sort_by_key(|(id, fighter)| fighter.get_stat(&Stat::Speed));
        self.fighters.reverse();

        // Get turns order
        let mut turn_order = Vec::new();
        self.fighters.iter().for_each(|(id, _)| turn_order.push(id.clone()));

        for id in turn_order {
            if !self.get_fighter(&id).alive { continue; }

            let rule = self.get_fighter(&id).get_rule(self);
            let action = rule.get_action();
            let target = action.get_target(&id, &self);

            action.execute(&id, &target, &mut self.fighters);
        }

        return None;
    }
}

pub const MAX_TURNS: u8 = 50;

#[test]
fn default_rule() {
    let f = fighter::dummy_fighter();
    let fighters = vec![(FighterID::Ally(0), f)];
    let fight = Fight { turn: 0, fighters };

    let (_, f0) = fight.fighters.get(0).unwrap();
    assert_eq!(&f0.get_rule(&fight), &AllRules::Default.new());
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![AllRules::Attack2.new()]);

    let fight = Fight { turn: 2, fighters: vec![(FighterID::Ally(0), f)] };

    let (_, fighter) = fight.fighters.get(0).unwrap();
    assert_eq!(&fighter.get_rule(&fight), &AllRules::Attack2.new());
}

#[test]
fn max_turns() {
    let team1 = vec![fighter::dummy_fighter()];
    let team2 = vec![fighter::dummy_foe()];

    assert_eq!(Fight::start(team1, team2), State::Draw);
}

#[test]
fn order_by_speed() {
    let mut fight = Fight {
        turn: 0,
        fighters: vec![
            (FighterID::Ally(0), fighter::dummy_foe()),
            (FighterID::Enemy(0), fighter::dummy_fighter())],
    };

    {
        let (_, f0) = fight.fighters.get(0).unwrap();
        let (_, f1) = fight.fighters.get(1).unwrap();
        assert_eq!(f0.get_stat(&Stat::Speed), 5);
        assert_eq!(f1.get_stat(&Stat::Speed), 10);
    }

    fight.turn();

    {
        let (_, f0) = fight.fighters.get(0).unwrap();
        let (_, f1) = fight.fighters.get(1).unwrap();
        assert_eq!(f0.get_stat(&Stat::Speed), 10);
        assert_eq!(f1.get_stat(&Stat::Speed), 5);
    }
}