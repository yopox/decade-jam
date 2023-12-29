use std::fmt::Debug;
use std::ops::Deref;
use crate::logic_prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub gate: Gate,
    pub action: Action,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gate {
    ID(Condition),
    NOT(Condition),
    AND(Condition, Condition),
    NAND(Condition, Condition),
    OR(Condition, Condition),
    XOR(Condition, Condition),
    NOR(Condition, Condition),
    NXOR(Condition, Condition),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    EveryXTurn(u8),
    OnTurn(u8),
    FromTurnX(u8),
    LessXHP(u8, Target),
    MoreXHP(u8, Target),
    HasStatus(Target, Status),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Attack(Target),
    Defense,
    Wait,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Them,
    AllyMost(Stat),
    AllyLess(Stat),
    FoeMost(Stat),
    FoeLess(Stat),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stat {
    Health,
    Attack,
    Defense,
    Speed,
    Nature,
    Demon,
}

impl Gate {
    pub fn check(&self, status: &Fight) -> bool {
        match self {
            Gate::ID(cond) => cond.check(status),
            Gate::NOT(cond) => !cond.check(status),
            Gate::AND(cond1, cond2) => cond1.check(status) && cond2.check(status),
            Gate::NAND(cond1, cond2) => !(cond1.check(status) && cond2.check(status)),
            Gate::OR(cond1, cond2) => cond1.check(status) || cond2.check(status),
            Gate::XOR(cond1, cond2) => {
                (cond1.check(status) && !cond2.check(status))
                    || (!cond1.check(status) && cond2.check(status))
            }
            Gate::NOR(cond1, cond2) => !(cond1.check(status) || cond2.check(status)),
            Gate::NXOR(cond1, cond2) => (!cond1.check(status) && !cond2.check(status))
                || (cond1.check(status) || cond2.check(status)),
        }
    }
}

impl Condition {
    pub fn check(&self, status: &Fight) -> bool {
        match self {
            Condition::EveryXTurn(x) => status.turn % x == 0,
            Condition::OnTurn(turn) => status.turn == *turn,
            Condition::FromTurnX(turn) => status.turn >= *turn,
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
            Action::Attack(target) => format!("Attack {:?}", target),
            Action::Defense => "Defense".to_string(),
            Action::Wait => "Wait".to_string(),
        }
    }
    pub fn get_target(&self, active: &FighterID, fight: &Fight) -> FighterID {
        match self {
            Action::Wait | Action::Defense => active.clone(),
            Action::Attack(target) => target.resolve(active, fight),
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
            Action::Attack(_) => {
                if let Some(weapon) = active.get_weapon() {
                    consequences.append(&mut weapon.use_weapon(active, target))
                }
            },
        }
        return consequences;
    }
}