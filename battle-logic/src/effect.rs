use crate::equipment::{AttackType, Element};
use crate::fighter::Fighter;
use crate::rune::Stat;

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    Attack {
        on_self: bool,
        attack_type: AttackType,
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
        consequence: Consequence,
    },
}

#[derive(Debug, PartialEq, Clone)]
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

impl Effect {
    pub fn apply_on_target(&self, source: &Fighter, target: &Fighter) -> Vec<(bool, Consequence)> {
        let mut consequences = vec![];
        if !target.is_alive() {
            return consequences;
        }

        match self {
            Effect::Attack { on_self, attack_type, element, damage } => {
                let attack_target = if *on_self { source } else { target };
                let (attack, defense) = match attack_type {
                    AttackType::Physical => (source.physical_attack(element), attack_target.physical_defense(element)),
                    AttackType::Magical => (source.magical_attack(element), attack_target.magical_defense(element)),
                };
                consequences.push((*on_self, Consequence::Attack { attack, defense, damage: *damage }))
            }
            Effect::Heal { .. } => {}
            Effect::Boost { on_self, consequence } => match consequence {
                Consequence::Buff { .. } => consequences.push((*on_self, consequence.clone())),
                _ => {}
            },
        }

        return consequences;
    }
}

impl Consequence {
    pub fn apply_on(&self, fighter: &mut Fighter) {
        match self {
            Consequence::Attack { attack, defense, damage } =>
                fighter.damage(*attack, *defense, *damage),
            Consequence::Buff { .. } => {}
        }
    }
}