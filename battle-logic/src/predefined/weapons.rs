use crate::logic_prelude::*;
use crate::predefined_prelude::*;

pub struct Sword {
    name: String,
    damage: Vec<(bool, Effect)>,
}

impl Weapon for Sword {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn use_weapon(&self, user: &Fighter, target: &Fighter) -> Vec<(bool, Consequence)> {
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
                (false, Effect::Attack {
                    damage: 10,
                    attack_type: AttackType::Physical,
                    element: Element::Neutral
                })
            ],
        },
    }
}
