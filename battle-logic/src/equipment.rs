use crate::effect::{Consequence, Effect};
use crate::fighter::Fighter;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Neutral,
    Demonic,
    Natural,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttackType {
    Physical,
    Magical,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Weapon {
    pub name: String,
    pub effects: Vec<Effect>,
}

pub trait Usable {
    fn get_effects(&self) -> &Vec<Effect>;

    fn use_on_target(&self, source: &Fighter, target: &Fighter) -> Vec<(bool, Consequence)> {
        let mut consequences = vec![];
        for effect in self.get_effects() {
            consequences.append(&mut effect.apply_on_target(source, target));
        }
        return consequences;
    }
}

impl Usable for Weapon {
    fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}
