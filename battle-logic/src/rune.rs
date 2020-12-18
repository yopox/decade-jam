use std::ops::Deref;

use crate::{fight, fighter};
use crate::effect::Consequence;
use crate::equipment;
use crate::equipment::Usable;
use crate::fight::{Fight, FighterID};

#[derive(Debug, PartialEq, Clone)]
pub enum Rule {
    Id(Condition, Action),
    Not(Condition, Action),
    And(Condition, Condition, Action),
    Nand(Condition, Condition, Action),
    Or(Condition, Condition, Action),
    Xor(Condition, Condition, Action),
    Nor(Condition, Condition, Action),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    EveryXTurn(u8),
    OnTurn(u8),
    LessXHP(u8, Target),
    MoreXHP(u8, Target),
    HasStatus(Target, fighter::Status),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Attack(equipment::Weapon, Target),
    Defense,
    Wait,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Them,
    AllyMost(Stat),
    AllyLess(Stat),
    FoeMost(Stat),
    FoeLess(Stat),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stat {
    Health,
    Attack,
    Defense,
    Wisdom,
    Speed,
    Nature,
    Demon,
}

impl Rule {
    pub fn check(&self, status: &Fight) -> bool {
        match self {
            Rule::Id(cond, _) => cond.check(status),
            Rule::Not(cond, _) => !cond.check(status),
            Rule::And(cond1, cond2, _) => cond1.check(status) && cond2.check(status),
            Rule::Nand(cond1, cond2, _) => !(cond1.check(status) && cond2.check(status)),
            Rule::Or(cond1, cond2, _) => cond1.check(status) || cond2.check(status),
            Rule::Xor(cond1, cond2, _) => {
                (cond1.check(status) && !cond2.check(status))
                    || (!cond1.check(status) && cond2.check(status))
            }
            Rule::Nor(cond1, cond2, _) => !(cond1.check(status) || cond2.check(status)),
        }
    }

    pub fn get_action(&self) -> &Action {
        match self {
            Rule::Id(_, action) => action,
            Rule::Not(_, action) => action,
            Rule::And(_, _, action) => action,
            Rule::Nand(_, _, action) => action,
            Rule::Or(_, _, action) => action,
            Rule::Xor(_, _, action) => action,
            Rule::Nor(_, _, action) => action,
        }
    }
}

impl Condition {
    pub fn check(&self, status: &Fight) -> bool {
        match self {
            Condition::EveryXTurn(x) => status.turn % x == 0,
            Condition::OnTurn(turn) => status.turn == *turn,
            Condition::LessXHP(_, _) => true,
            Condition::MoreXHP(_, _) => true,
            Condition::HasStatus(_, _) => true,
        }
    }
}

impl Target {
    pub fn resolve(&self, active: &fight::FighterID, fight: &fight::Fight) -> fight::FighterID {
        // TODO: Optimize by filtering only if needed
        let allies = fight
            .fighters
            .iter()
            .filter(|(id, _)| id.is_ally() == active.is_ally());
        let enemies = fight
            .fighters
            .iter()
            .filter(|(id, _)| id.is_ally() != active.is_ally());

        match self {
            Target::Them => active.clone(),
            Target::AllyMost(stat) => {
                match allies.max_by_key(|(_, f)| f.borrow().deref().get_stat(stat)) {
                    Some((id, _)) => id.clone(),
                    None => FighterID::None,
                }
            }
            Target::AllyLess(stat) => {
                match allies.min_by_key(|(_, f)| f.borrow().deref().get_stat(stat)) {
                    Some((id, _)) => id.clone(),
                    None => FighterID::None,
                }
            }
            Target::FoeMost(stat) => {
                match enemies.max_by_key(|(_, f)| f.borrow().deref().get_stat(stat)) {
                    Some((id, _)) => id.clone(),
                    None => FighterID::None,
                }
            }
            Target::FoeLess(stat) => {
                match enemies.min_by_key(|(_, f)| f.borrow().deref().get_stat(stat)) {
                    Some((id, _)) => id.clone(),
                    None => FighterID::None,
                }
            }
        }
    }
}

impl Action {
    pub fn get_target(&self, active: &fight::FighterID, fight: &fight::Fight) -> fight::FighterID {
        match self {
            Action::Wait | Action::Defense => active.clone(),
            Action::Attack(_, target) => target.resolve(active, fight),
        }
    }

    pub fn execute(&self, active: &fighter::Fighter, target: &fighter::Fighter) -> Vec<(bool, Consequence)> {
        println!(
            "\t\t{:?} ({:}).",
            self,
            target.get_name()
        );
        let mut consequences = vec![];
        match self {
            Action::Wait => (),
            Action::Defense => consequences.push((true, active.defense())),
            Action::Attack(weapon, _) => consequences.append(&mut weapon.use_on_target(active, target)),
        }
        return consequences;
    }
}