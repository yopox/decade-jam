use crate::fighter::Fighter;
use crate::runes::Stat;

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    PhysicalAttack { on_self: bool, damage: u16 },
    MagicalAttack { on_self: bool, damage: u16 },
    Heal { on_self: bool, amount: u16, duration: u8 },
    Boost { on_self: bool, stat: Stat, amount: i16, duration: u8 },
}

impl Effect {

    pub fn on_self(&self) -> bool {
        *match self {
            Effect::PhysicalAttack { on_self, damage: _ } => on_self,
            Effect::MagicalAttack { on_self, damage: _ } => on_self,
            Effect::Heal { on_self, amount: _, duration: _ } => on_self,
            Effect::Boost { on_self, stat: _, amount: _, duration: _ } => on_self,
        }
    }

    pub fn apply(&self, source: &mut Fighter, target: &mut Fighter) {
        match self {
            Effect::PhysicalAttack { on_self, damage } => {
                target.damage(*damage);
            }
            Effect::MagicalAttack { on_self, damage } => {}
            Effect::Heal { on_self, amount, duration } => {}
            Effect::Boost { on_self, stat, amount, duration } => {}
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