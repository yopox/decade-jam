use crate::fighter;
use crate::predefined::rules::AllRules;
use crate::rune;
use crate::rune::Rule;

pub mod rules {
    use crate::predefined::*;
    use crate::predefined::weapons::AllWeapons;
    use crate::rune::*;

    pub enum AllRules {
        Default,
        Defense,
        Attack2,
        Careful,
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
        }
    }
}

impl Default for Rule {
    fn default() -> Self { AllRules::Default.new() }
}

pub mod weapons {
    use crate::effect::{Consequence, Effect};
    use crate::equipment::AttackType::{Magical, Physical};
    use crate::equipment::Element::{Demonic, Natural};
    use crate::equipment::Weapon;
    use crate::rune::Stat;

    pub enum AllWeapons {
        WoodenSword,
        FireRod,
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
                effects: vec![Effect::Attack {
                    on_self: false,
                    attack_type: Physical,
                    element: Natural,
                    damage: 10,
                }],
            },
            AllWeapons::FireRod => Weapon {
                name: String::from("Fire Rod"),
                effects: vec![Effect::Attack {
                    on_self: false,
                    attack_type: Magical,
                    element: Demonic,
                    damage: 15,
                }, Effect::Boost {
                    on_self: false,
                    consequence: Consequence::Buff {
                        stat: Stat::Defense,
                        amount: -5,
                        duration: 1,
                    },
                }],
            },
        }
    }
}