use crate::logic::prelude::*;

#[derive(Clone)]
pub enum Element {
    Neutral,
    Demonic,
    Natural,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WeaponTarget {
    Me,
    Other,
}

/// TODO: Make [Weapon] a rune in [crate::logic::rule]
pub trait Weapon {
    fn get_name(&self) -> String;
    fn use_weapon(&self, user: &Fighter, target: &Fighter) -> Vec<(WeaponTarget, Consequence)>;
}

#[derive(Clone)]
pub enum Consequence {
    Attack {
        damage: u16,
    },
    Buff {
        stat: Stat,
        amount: i32,
        duration: u8,
    },
}

impl Consequence {
    pub fn apply_on(&self, fighter: &mut Fighter) {
        match self {
            Consequence::Attack { damage } => fighter.damage(*damage),
            Consequence::Buff { .. } => {}
        }
    }

    pub fn from_damage(element: &Element, damage: u16, user: &Fighter) -> Consequence {
        let attack = user.calc_attack(element);
        Consequence::Attack { damage: attack + damage }
    }
}