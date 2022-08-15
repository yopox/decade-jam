use crate::logic_prelude::*;

pub enum Effect {
    Attack {
        damage: u16,
        element: Element,
    }
}

impl Effect {
    pub fn to_consequence(&self, user: &Fighter, target: &Fighter) -> Consequence {
        match self {
            Effect::Attack { damage, element } => {
                Consequence::from_damage(element, *damage, user)
            }
        }
    }
}