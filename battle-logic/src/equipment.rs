use crate::fighter::Fighter;
use crate::runes::Stat;

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    PhysicalAttack { on_self: bool, damage: u16 },
    MagicalAttack { on_self: bool, damage: u16 },
    Heal { on_self: bool, amount: u16, duration: u8 },
    Boost { on_self: bool, stat: Stat, amount: i16, duration: u8 },
    Leech { stat: Stat, amount: u16, duration: u8 },
}

impl Effect {
    pub fn apply(&self, source: &mut Fighter, target: &mut Fighter) {
        match self {
            Effect::PhysicalAttack { on_self, damage } => {
                target.damage(*damage);
            }
            Effect::MagicalAttack { on_self, damage } => {}
            Effect::Heal { on_self, amount, duration } => {}
            Effect::Boost { on_self, stat, amount, duration } => {}
            Effect::Leech { stat, amount, duration } => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: String,
    pub effects: Vec<Effect>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spell {
    pub name: String,
    pub effects: Vec<Effect>,
}