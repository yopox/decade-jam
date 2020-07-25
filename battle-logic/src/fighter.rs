use crate::{runes, fight};
use crate::runes::Rule;

struct Stats {
    hp: u16,
    mp: u16,
    attack: i16,
    defense: i16,
    holy: i16,
    evil: i16,
    speed: i16
}

pub enum Team { Ally, Enemy }

pub struct Fighter {
    name: String,
    stats: Stats,
    alive: bool,
    rules: [Option<runes::Rule>; 5],
    team: Team
}

impl Fighter {

    pub fn get_rule(&self, status: &fight::Status) -> &Rule {
        for rule in &self.rules {
            match rule {
                Some(rule) => if rule.check(status) { return rule }
                None => {}
            }
        }
        return &runes::predefined::DEFAULT;
    }

}

pub fn dummy_fighter() -> Fighter {
    Fighter {
        name: "Arches".to_string(),
        stats: Stats {
            hp: 10,
            mp: 0,
            attack: 10,
            defense: 0,
            holy: 0,
            evil: 0,
            speed: 10
        },
        alive: true,
        rules: [None, None, None, None, None],
        team: Team::Ally
    }
}