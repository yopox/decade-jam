use crate::fighter::Fighter;
use crate::runes::Stat;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Neutral,
    Demonic,
    Natural
}

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    PhysicalAttack {
        on_self: bool,
        element: Element,
        damage: u16,
    },
    MagicalAttack {
        on_self: bool,
        element: Element,
        damage: u16,
    },
    Heal {
        on_self: bool,
        amount: u16,
        duration: u8,
    },
    Boost {
        on_self: bool,
        stat: Stat,
        amount: i16,
        duration: u8,
    },
}

impl Effect {
    pub fn apply_on_self(&self, source: &mut Fighter) {
        if !source.is_alive() {
            return;
        }

        *match self {
            Effect::PhysicalAttack { on_self, element, damage: _ } => (),
            Effect::MagicalAttack { on_self, element, damage: _ } => (),
            Effect::Heal {
                on_self,
                amount: _,
                duration: _,
            } => (),
            Effect::Boost {
                on_self,
                stat: _,
                amount: _,
                duration: _,
            } => (),
        }
    }

    pub fn apply_on_target(&self, source: &mut Fighter, target: &mut Fighter) {
        if !target.is_alive() {
            return;
        }

        match self {
            Effect::PhysicalAttack { on_self, element, damage } => {

                target.damage(*damage);
                target.check_hp();
            }
            Effect::MagicalAttack { on_self, element, damage } => {}
            Effect::Heal {
                on_self,
                amount,
                duration,
            } => {}
            Effect::Boost {
                on_self,
                stat,
                amount,
                duration,
            } => {}
        }
    }
}

pub trait Usable {
    fn get_effects(&self) -> &Vec<Effect>;

    fn use_on_target(&self, source: &mut Fighter, target: &mut Fighter) {
        for effect in self.get_effects() {
            effect.apply_on_target(source, target);
        }
    }

    fn use_on_self(&self, source: &mut Fighter) {
        for effect in self.get_effects() {
            effect.apply_on_self(source);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: String,
    pub effects: Vec<Effect>,
}

impl Usable for Weapon {
    fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spell {
    pub name: String,
    pub effects: Vec<Effect>,
}

impl Usable for Spell {
    fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}