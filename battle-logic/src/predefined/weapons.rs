use crate::logic_prelude::*;
use crate::predefined_prelude::*;

pub enum Weapons {
    WoodenSword,
    FireRod,
}

impl Weapons {
    pub fn new(self) -> Weapon {
        get(self)
    }
}

fn get(name: Weapons) -> Weapon {
    match name {
        Weapons::WoodenSword => Weapon {
            name: String::from("Wooden Sword"),
            effects: vec![Effect::Attack {
                on_self: false,
                attack_type: AttackType::Physical,
                element: Element::Natural,
                damage: 10,
            }],
        },
        Weapons::FireRod => Weapon {
            name: String::from("Fire Rod"),
            effects: vec![Effect::Attack {
                on_self: false,
                attack_type: AttackType::Magical,
                element: Element::Demonic,
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
