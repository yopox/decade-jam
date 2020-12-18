use std::cell::{BorrowMutError, Ref, RefCell, RefMut};
use std::iter::Filter;
use std::ops::{Deref, DerefMut};
use std::slice::Iter;

use crate::effect::Consequence;
use crate::fighter;
use crate::fighter::{dummy_fighter, dummy_foe, Fighter};
use crate::predefined::rules::AllRules;
use crate::rune;
use crate::rune::{Rule, Stat, Target};

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

pub const MAX_TURNS: u8 = 50;

pub struct Fight {
    pub turn: u8,
    pub fighters: Vec<(FighterID, RefCell<fighter::Fighter>)>,
}

impl Fight {
    pub fn start(team1: Vec<Fighter>, team2: Vec<Fighter>) -> State {
        let mut fight = Fight::build_fight(team1, team2);

        loop {
            match fight.turn() {
                Some(result) => return result,
                None => (),
            }
        }
    }

    pub fn build_fight(team1: Vec<Fighter>, team2: Vec<Fighter>) -> Fight {
        let mut fighters = Vec::new();
        team1
            .into_iter()
            .enumerate()
            .for_each(|(i, f)| fighters.push((FighterID::Ally(i), RefCell::new(f))));
        team2
            .into_iter()
            .enumerate()
            .for_each(|(i, f)| fighters.push((FighterID::Enemy(i), RefCell::new(f))));

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
        self.fighters
            .iter()
            .for_each(|(id, _)| turn_order.push(id.clone()));

        let mut state: Option<State> = None;

        for id in turn_order {
            let (action, target) = {
                let mut active = self.get_fighter_mut(id);
                if !active.is_alive() { continue; };

                // Start of turn logic
                active.turn();

                // Resolve rule, action, target for the turn
                let rule = active.get_rule(&self);
                let action = rule.get_action().clone();
                let target = action.get_target(&id, &self);
                (action, target)
            };

            let consequences = {
                action.execute(self.get_fighter(id).deref(), self.get_fighter(target).deref())
            };

            for (on_self, consequence) in consequences {
                match on_self {
                    true => consequence.apply_on(self.get_fighter_mut(id).deref_mut()),
                    _ => consequence.apply_on(self.get_fighter_mut(target).deref_mut()),
                };
            }

            state = self.check_state();
            if state != None {
                break;
            }
        }

        return state;
    }

    fn get_fighter(&self, id: FighterID) -> Ref<Fighter> {
        self.fighters.iter().find(|(f_id, _)| *f_id == id).unwrap().1.borrow()
    }

    fn get_fighter_mut(&self, id: FighterID) -> RefMut<Fighter> {
        self.fighters.iter().find(|(f_id, _)| *f_id == id).unwrap().1.borrow_mut()
    }

    pub fn check_state(&self) -> Option<State> {
        if self.fighters
            .iter()
            .filter(|(id, _)| !id.is_ally())
            .all(|(_, f)| !f.borrow().is_alive())
        {
            return Some(State::AlliesVictory);
        } else if self.fighters
            .iter()
            .filter(|(id, _)| id.is_ally())
            .all(|(_, f)| !f.borrow().is_alive())
        {
            return Some(State::EnemiesVictory);
        }
        return None;
    }
}

#[test]
fn default_rule() {
    let fight = Fight::build_fight(vec![fighter::dummy_fighter()], vec![]);

    let (_, f0) = fight.fighters.get(0).unwrap();
    assert_eq!(
        &f0.borrow().deref().get_rule(&fight),
        &AllRules::Default.new()
    );
}

#[test]
fn every_two_turn() {
    let mut f = fighter::dummy_fighter();
    f.set_rules(vec![AllRules::Attack2.new()]);

    let mut fight = Fight::build_fight(vec![f], vec![]);
    fight.turn = 2;

    let (_, fighter) = fight.fighters.get(0).unwrap();
    assert_eq!(
        &fighter.borrow().deref().get_rule(&fight),
        &AllRules::Attack2.new()
    );
}

#[test]
fn max_turns() {
    let team1 = vec![fighter::dummy_fighter()];
    let team2 = vec![fighter::dummy_foe()];

    assert_eq!(Fight::start(team1, team2), State::Draw);
}

#[test]
fn order_by_speed() {
    let mut fight = Fight::build_fight(vec![dummy_fighter()], vec![dummy_foe()]);
    let foe = dummy_foe().get_name().clone();
    let fighter = dummy_fighter().get_name().clone();

    {
        let (_, f0) = fight.fighters.get(0).unwrap();
        let (_, f1) = fight.fighters.get(1).unwrap();
        assert_eq!(f0.borrow().deref().get_name(), &fighter);
        assert_eq!(f1.borrow().deref().get_name(), &foe);
    }

    fight.turn();

    {
        let (_, f0) = fight.fighters.get(0).unwrap();
        let (_, f1) = fight.fighters.get(1).unwrap();
        assert_eq!(f0.borrow().deref().get_name(), &foe);
        assert_eq!(f1.borrow().deref().get_name(), &fighter);
    }
}
