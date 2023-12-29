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
        Rules::Wait => Rule {
            gate: Gate::ID(Condition::EveryXTurn(1)),
            action: Action::Wait
        },
        Rules::Defense => Rule {
            gate: Gate::ID(Condition::EveryXTurn(1)),
            action: Action::Defense
        },
        Rules::Attack2 => Rule {
            gate: Gate::ID(Condition::EveryXTurn(2)),
            action: Action::Attack(Target::FoeLess(Stat::Health)),
        },
        Rules::Careful => Rule {
            gate: Gate::AND(
                Condition::EveryXTurn(2),
                Condition::LessXHP(30, Target::Them)
            ),
            action: Action::Defense
        },
    }
}

impl Default for Rule {
    fn default() -> Self { Rules::Wait.new() }
}
