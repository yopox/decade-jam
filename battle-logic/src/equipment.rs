use crate::runes::Stat;

#[derive(Debug, PartialEq)]
pub enum Effect {
    PhysicalAttack { on_self: bool, damage: u8 },
    MagicalAttack { on_self: bool, damage: u8 },
    Heal { on_self: bool, amount: u8, duration: u8 },
    Boost { on_self: bool, stat: Stat, amount: i16, duration: u8 },
    Leech { stat: Stat, amount: u16, duration: u8 },
}

#[derive(Debug, PartialEq)]
pub struct Weapon {
    pub name: String,
    pub effects: Box<[Effect]>,
}

#[derive(Debug, PartialEq)]
pub struct Spell {
    pub name: String,
    pub effects: Box<[Effect]>,
}