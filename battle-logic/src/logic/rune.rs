use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use crate::logic_prelude::*;

#[derive(Clone)]
pub enum Rule {
    ID(Condition, Action),
    NOT(Condition, Action),
    AND(Condition, Condition, Action),
    NAND(Condition, Condition, Action),
    OR(Condition, Condition, Action),
    XOR(Condition, Condition, Action),
    NOR(Condition, Condition, Action),
    NXOR(Condition, Condition, Action),
}

#[derive(Clone)]
pub enum Condition {
    EveryXTurn(u8),
    OnTurn(u8),
    LessXHP(u8, Target),
    MoreXHP(u8, Target),
    HasStatus(Target, Status),
}

#[derive(Clone)]
pub enum Action {
    Attack(Rc<dyn Weapon>, Target),
    Defense,
    Wait,
}

#[derive(Debug, Clone)]
pub enum Target {
    Them,
    AllyMost(Stat),
    AllyLess(Stat),
    FoeMost(Stat),
    FoeLess(Stat),
}

#[derive(Debug, Clone)]
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
            Rule::ID(cond, _) => cond.check(status),
            Rule::NOT(cond, _) => !cond.check(status),
            Rule::AND(cond1, cond2, _) => cond1.check(status) && cond2.check(status),
            Rule::NAND(cond1, cond2, _) => !(cond1.check(status) && cond2.check(status)),
            Rule::OR(cond1, cond2, _) => cond1.check(status) || cond2.check(status),
            Rule::XOR(cond1, cond2, _) => {
                (cond1.check(status) && !cond2.check(status))
                    || (!cond1.check(status) && cond2.check(status))
            }
            Rule::NOR(cond1, cond2, _) => !(cond1.check(status) || cond2.check(status)),
            Rule::NXOR(cond1, cond2, _) => (!cond1.check(status) && !cond2.check(status))
                || (cond1.check(status) || cond2.check(status)),
        }
    }

    pub fn get_action(&self) -> &Action {
        match self {
            Rule::ID(_, action) => action,
            Rule::NOT(_, action) => action,
            Rule::AND(_, _, action) => action,
            Rule::NAND(_, _, action) => action,
            Rule::OR(_, _, action) => action,
            Rule::XOR(_, _, action) => action,
            Rule::NOR(_, _, action) => action,
            Rule::NXOR(_, _, action) => action,
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
    pub fn resolve(&self, active: &FighterID, fight: &Fight) -> FighterID {
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
    pub fn name(&self) -> String {
        match self {
            Action::Attack(_, target) => format!("Attack {:?}", target),
            Action::Defense => "Defense".to_string(),
            Action::Wait => "Wait".to_string(),
        }
    }
    pub fn get_target(&self, active: &FighterID, fight: &Fight) -> FighterID {
        match self {
            Action::Wait | Action::Defense => active.clone(),
            Action::Attack(_, target) => target.resolve(active, fight),
        }
    }

    pub fn execute(&self, active: &Fighter, target: &Fighter) -> Vec<(WeaponTarget, Consequence)> {
        println!(
            "\t\t{:} ({:}).",
            self.name(),
            target.get_name()
        );
        let mut consequences = vec![];
        match self {
            Action::Wait => (),
            Action::Defense => consequences.push((WeaponTarget::Me, active.defense())),
            Action::Attack(weapon, _) => consequences.append(&mut weapon.use_weapon(active, target)),
        }
        return consequences;
    }
}