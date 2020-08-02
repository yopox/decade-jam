use crate::fighter;
use crate::runes;

pub mod rules {
    use crate::predefined::*;
    use crate::runes::*;

    pub const DEFAULT: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Wait);

    pub const DEFENSE: Rule = Rule::Id(Condition::EveryXTurn(1), Action::Defense);

    pub const ATTACK_2: Rule = Rule::Id(
        Condition::EveryXTurn(2),
        Action::Attack(weapons::WOODEN_SWORD, Target::FoeLess(Stat::Health)),
    );

    pub const CAREFUL: Rule = Rule::And(
        Condition::EveryXTurn(2),
        Condition::LessXHP(30, Target::Them),
        Action::Defense,
    );

    pub const MAGICIAN: Rule = Rule::Id(
        Condition::EveryXTurn(3),
        Action::Spell(spells::FIREBALL, Target::FoeLess(Stat::Health)),
    );
}

pub mod weapons {
    use crate::fighter::Weapon;

    pub const WOODEN_SWORD: Weapon = Weapon {};
}

pub mod spells {
    use crate::fighter::Spell;

    pub const FIREBALL: Spell = Spell {};
}