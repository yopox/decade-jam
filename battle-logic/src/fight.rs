use std::iter::Filter;
use std::slice::Iter;

use crate::fighter;
use crate::fighter::{Fighter, dummy_foe, dummy_fighter};
use crate::predefined;
use crate::predefined::rules::AllRules;
use crate::predefined::spells::AllSpells;
use crate::predefined::weapons::AllWeapons;
use crate::runes;
use crate::runes::{Stat, Target};
use std::cell::{RefCell, Ref, BorrowMutError, RefMut};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

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
    pub fighters: Vec<(FighterID, RefCell<fighter::Fighter>)>,
}

impl Fight {
    pub fn start(mut team1: Vec<Fighter>, mut team2: Vec<Fighter>) -> State {
        let mut fight = Fight::build_fight(team1, team2);

        loop {
            match fight.turn() {
                Some(result) => return result,
                None => (),
            }
        }
    }

    pub fn build_fight(mut team1: Vec<Fighter>, mut team2: Vec<Fighter>) -> Fight {
        let mut fighters = Vec::new();
        team1.into_iter().enumerate().for_each(|(i, f)| fighters.push(
            (FighterID::Ally(i), RefCell::new(f))
        ));
        team2.into_iter().enumerate().for_each(|(i, f)| fighters.push(
            (FighterID::Enemy(i), RefCell::new(f))
        ));

        Fight { turn: 0, fighters }
    }

    pub fn turn(&mut self) -> Option<State> {
        self.turn += 1;
        if self.turn > MAX_TURNS {
            return Some(State::Draw);
        }

        println!("Turn {}", self.turn);

        // Order fighters by speed
        self.fighters.sort_by_key(|(id, fighter)| fighter.borrow().deref().get_stat(&Stat::Speed));
        self.fighters.reverse();

        // Get turns order
        let mut turn_order = Vec::new();
        self.fighters.iter().for_each(|(id, _)| turn_order.push(id.clone()));

        for id in turn_order {

            // Resolve rule, action, target for the turn
            let rule = {
                let active = match self.fighters.iter().find(|(fID, _)| *fID == id) {
                    Some((_, x)) => x.borrow(),
                    None => panic!("Tried to get &Fighter from wrong FighterID."),
                };

                if !active.alive { continue; }
                active.deref().get_rule(self)
            };

            let action = rule.get_action();
            let target = action.get_target(&id, &self);

            // Execute the action

            let mut active = match self.fighters.iter().find(|(fID, _)| *fID == id) {
                Some((_, x)) => match x.try_borrow_mut() {
                    Ok(mut f) => f,
                    Err(_) => panic!("Active fighter already borrowed."),
                },
                None => panic!(),
            };

            if target == id {
                // Action on self (1 fighter)
                action.execute_self(active.deref_mut());
            } else {
                // Action on a different fighter (2 fighters)
                let mut target =  match self.fighters.iter().find(|(fID, _)| *fID == target) {
                    Some((_, x)) => match x.try_borrow_mut() {
                        Ok(mut f) => f,
                        Err(_) => panic!("Active fighter already borrowed."),
                    },
                    None => panic!(),
                };
                action.execute(active.deref_mut(), target.deref_mut());
            }
        }

        return None;
    }
}

pub const MAX_TURNS: u8 = 50;

#[test]
fn default_rule() {
    let fight = Fight::build_fight(vec![fighter::dummy_fighter()], vec![]);

    let (_, f0) = fight.fighters.get(0).unwrap();
    assert_eq!(&f0.get_rule(&fight), &AllRules::Default.new());
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![AllRules::Attack2.new()]);

    let mut fight = Fight::build_fight(vec![f], vec![]);
    fight.turn = 2;

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
    let mut fight = Fight::build_fight(vec![dummy_foe()], vec![dummy_fighter()]);

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