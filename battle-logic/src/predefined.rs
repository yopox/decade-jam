use crate::fighter;
use crate::runes;

pub mod rules {
    use crate::predefined::*;
    use crate::predefined::spells::AllSpells;
    use crate::predefined::weapons::AllWeapons;
    use crate::runes::*;

    pub enum AllRules {
        Default,
        Defense,
        Attack2,
        Careful,
        Magician,
    }

    impl AllRules {
        pub fn new(self) -> Rule {
            get(self)
        }
    }

    fn get(name: AllRules) -> Rule {
        match name {
            AllRules::Default => Rule::Id(Condition::EveryXTurn(1), Action::Wait),
            AllRules::Defense => Rule::Id(Condition::EveryXTurn(1), Action::Defense),
            AllRules::Attack2 => Rule::Id(
                Condition::EveryXTurn(2),
                Action::Attack(AllWeapons::WoodenSword.new(), Target::FoeLess(Stat::Health)),
            ),
            AllRules::Careful => Rule::And(
                Condition::EveryXTurn(2),
                Condition::LessXHP(30, Target::Them),
                Action::Defense,
            ),
            AllRules::Magician => Rule::Id(
                Condition::EveryXTurn(3),
                Action::Spell(AllSpells::Fireball.new(), Target::FoeLess(Stat::Health)),
            ),
        }
    }
}

pub mod weapons {
    use crate::equipment::{Effect, Weapon};

    pub enum AllWeapons {
        WoodenSword
    }

    impl AllWeapons {
        pub fn new(self) -> Weapon {
            get(self)
        }
    }

    fn get(name: AllWeapons) -> Weapon {
        match name {
            AllWeapons::WoodenSword => Weapon {
                name: String::from("Wooden Sword"),
                effects: vec![Effect::PhysicalAttack { on_self: false, damage: 5 }],
            }
        }
    }
}

pub mod spells {
    use crate::equipment::{Effect, Spell};

    pub enum AllSpells {
        Fireball
    }

    impl AllSpells {
        pub fn new(self) -> Spell {
            get(self)
        }
    }

    fn get(name: AllSpells) -> Spell {
        match name {
            AllSpells::Fireball => Spell {
                name: String::from("Fireball"),
                effects: vec![Effect::PhysicalAttack { on_self: false, damage: 5 }],
            }
        }
    }
}