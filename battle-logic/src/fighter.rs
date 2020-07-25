use crate::runes::Rule;
use crate::{fight, runes};

struct Stats {
    hp: u16,
    mp: u16,
    attack: i16,
    defense: i16,
    holy: i16,
    evil: i16,
    speed: i16,
}

#[derive(Debug, PartialEq)]
pub enum Team {
    Ally,
    Enemy,
}

pub struct Fighter {
    name: String,
    stats: Stats,
    alive: bool,
    rules: Vec<runes::Rule>,
    team: Team,
}

impl Fighter {
    pub fn get_rule(&self, status: &fight::Status) -> &Rule {
        for rule in &self.rules {
            if rule.check(status) {
                return rule;
            }
        }
        return &runes::predefined::DEFAULT;
    }

    pub fn set_rules(&mut self, rules: Vec<runes::Rule>) {
        self.rules = rules;
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
            speed: 10,
        },
        alive: true,
        rules: Vec::new(),
        team: Team::Ally,
    }
}

pub fn dummy_foe() -> Fighter {
    let mut foe = dummy_fighter();
    foe.team = Team::Enemy;
    return foe;
}
