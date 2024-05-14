use crate::logic::prelude::*;
use crate::predefined::prelude::*;

pub struct Sword {
    name: String,
    damage: Vec<(WeaponTarget, Effect)>,
}

impl Weapon for Sword {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn use_weapon(&self, user: &Fighter, target: &Fighter) -> Vec<(WeaponTarget, Consequence)> {
        let mut consequences  = vec![];
        for (b, e) in &self.damage {
            consequences.push((b.clone(), e.to_consequence(user, target)))
        }
        consequences
    }
}

pub enum Swords {
    WoodenSword,
}

impl Swords {
    pub fn new(self) -> Sword {
        get(self)
    }
}

fn get(name: Swords) -> Sword {
    match name {
        Swords::WoodenSword => Sword {
            name: String::from("Wooden Sword"),
            damage: vec![
                (WeaponTarget::Other, Effect::Attack {
                    damage: 10,
                    element: Element::Neutral
                })
            ],
        },
    }
}
