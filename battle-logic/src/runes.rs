#[derive(Debug)]
pub enum Rule {
    Id(Condition, Action),
    Not(Condition, Action),
    And(Condition, Condition, Action),
    Nand(Condition, Condition, Action),
    Or(Condition, Condition, Action),
    Xor(Condition, Condition, Action),
    Nor(Condition, Condition, Action),
}

#[derive(Debug)]
pub enum Condition {
    EveryXTurn(u16),
    LessXHP(u16, Target),
    MoreXHP(u16, Target),
}

#[derive(Debug)]
pub enum Action {
    Attack(Target),
    Defense,
    Spell(Target),
    Wait,
}

#[derive(Debug)]
pub enum Target {
    Them,
    AllyMost(Stat),
    AllyLess(Stat),
    FoeMost(Stat),
    FoeLess(Stat),
}

#[derive(Debug)]
pub enum Stat {
    HP,
    Attack,
    Defense,
    Wisdom,
    Speed,
}

pub mod predefined {
    use crate::runes::*;

    pub const DEFAULT: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Wait);

    pub const DEFENSE: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Defense);

    pub const CAREFUL: Rule = Rule::And(
        Condition::EveryXTurn(2),
        Condition::LessXHP(30, Target::Them),
        Action::Defense);

    pub const MAGICIAN: Rule = Rule::Id(
        Condition::EveryXTurn(3), Action::Spell(Target::FoeLess(Stat::HP)),
    );
}