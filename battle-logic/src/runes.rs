use crate::fight::Status;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Id(Condition, Action),
    Not(Condition, Action),
    And(Condition, Condition, Action),
    Nand(Condition, Condition, Action),
    Or(Condition, Condition, Action),
    Xor(Condition, Condition, Action),
    Nor(Condition, Condition, Action),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    EveryXTurn(u8),
    LessXHP(u8, Target),
    MoreXHP(u8, Target),
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Attack(Target),
    Defense,
    Spell(Target),
    Wait,
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Them,
    AllyMost(Stat),
    AllyLess(Stat),
    FoeMost(Stat),
    FoeLess(Stat),
}

#[derive(Debug, PartialEq)]
pub enum Stat {
    HP,
    Attack,
    Defense,
    Wisdom,
    Speed,
}

impl Rule {
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

    pub fn check(&self, status: &Status) -> bool {
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
}

impl Condition {
    pub fn check(&self, status: &Status) -> bool {
        match self {
            Condition::EveryXTurn(x) => status.turn % x == 0,
            Condition::LessXHP(_, _) => true,
            Condition::MoreXHP(_, _) => true,
        }
    }
}

pub mod predefined {
    use crate::runes::*;

    pub const DEFAULT: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Wait);

    pub const DEFENSE: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Defense);

    pub const ATTACK_2: Rule = Rule::Id(
        Condition::EveryXTurn(2),
        Action::Attack(Target::FoeLess(Stat::HP)),
    );

    pub const CAREFUL: Rule = Rule::And(
        Condition::EveryXTurn(2),
        Condition::LessXHP(30, Target::Them),
        Action::Defense,
    );

    pub const MAGICIAN: Rule = Rule::Id(
        Condition::EveryXTurn(3),
        Action::Spell(Target::FoeLess(Stat::HP)),
    );
}
