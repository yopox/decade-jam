use crate::logic_prelude::*;
use crate::predefined_prelude::*;

pub enum Rules {
    Wait,
    Defense,
    Attack2,
    Careful,
}

impl Rules {
    pub fn new(self) -> Rule {
        get(self)
    }
}

fn get(name: Rules) -> Rule {
    match name {
        Rules::Wait => Rule::ID(Condition::EveryXTurn(1), Action::Wait),
        Rules::Defense => Rule::ID(Condition::EveryXTurn(1), Action::Defense),
        Rules::Attack2 => Rule::ID(
            Condition::EveryXTurn(2),
            Action::Attack(Weapons::WoodenSword.new(), Target::FoeLess(Stat::Health)),
        ),
        Rules::Careful => Rule::AND(
            Condition::EveryXTurn(2),
            Condition::LessXHP(30, Target::Them),
            Action::Defense,
        ),
    }
}

impl Default for Rule {
    fn default() -> Self { Rules::Wait.new() }
}
