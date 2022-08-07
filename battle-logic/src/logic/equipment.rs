use crate::logic_prelude::*;

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

pub trait Weapon {
    fn get_name(&self) -> String;
    fn use_weapon(&self, user: &Fighter, target: &Fighter) -> Vec<(WeaponTarget, Consequence)>;
}

#[derive(Clone)]
pub enum Consequence {
    Attack {
        attack: u16,
        defense: u16,
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
            Consequence::Attack { attack, defense, damage } => fighter.damage(*attack, *defense, *damage),
            Consequence::Buff { .. } => {}
        }
    }

    pub fn from_damage(element: &Element, damage: u16, user: &Fighter, target: &Fighter) -> Consequence {
        let (attack, defense) = (user.calc_attack(element), target.calc_defense(element));
        Consequence::Attack { attack, defense, damage }
    }
}