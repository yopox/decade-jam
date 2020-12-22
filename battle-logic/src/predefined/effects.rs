use crate::logic_prelude::*;

pub enum Effect {
    Attack {
        damage: u16,
        attack_type: AttackType,
        element: Element,
    }
}

impl Effect {
    pub fn to_consequence(&self, user: &Fighter, target: &Fighter) -> Consequence {
        match self {
            Effect::Attack { damage, attack_type, element } => {
                Consequence::from_damage(attack_type, element, *damage, user, target)
            }
        }
    }
}