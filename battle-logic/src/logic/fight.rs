use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut};

use crate::logic::prelude::*;

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
    pub fighters: Vec<(FighterID, RefCell<Fighter>)>,
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
        self.fighters.sort_by_key(|(_, fighter)| fighter.borrow().deref().get_stat(&Stat::Speed));
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
                let action = rule.action.clone();
                let target = action.get_target(&id, &self);
                (action, target)
            };

            let consequences = {
                action.execute(self.get_fighter(id).deref(), self.get_fighter(target).deref())
            };

            for (on_self, consequence) in consequences {
                match on_self {
                    WeaponTarget::Me => consequence.apply_on(self.get_fighter_mut(id).deref_mut()),
                    WeaponTarget::Other => consequence.apply_on(self.get_fighter_mut(target).deref_mut()),
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
