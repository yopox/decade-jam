use crate::runes::{Rule, Stat};
use crate::{fight, runes};

struct Stats {
    health: u16,
    mana: u16,
    attack: u16,
    defense: u16,
    wisdom: u16,
    speed: u16,
    nature: u16,
    demon: u16,
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
    pub fn get_stat(&self, stat: runes::Stat) -> u16 {
        match stat {
            Stat::Health => self.stats.health,
            Stat::Mana => self.stats.mana,
            Stat::Attack => self.stats.attack,
            Stat::Defense => self.stats.defense,
            Stat::Wisdom => self.stats.wisdom,
            Stat::Speed => self.stats.speed,
            Stat::Nature => self.stats.nature,
            Stat::Demon => self.stats.demon,
        }
    }

    pub fn get_rule(&self, status: &fight::Status) -> &Rule {
        return match self.rules.iter().find(|rule| rule.check(status)) {
            Some(rule) => rule,
            None => &runes::predefined::DEFAULT,
        }
    }

    pub fn set_rules(&mut self, rules: Vec<runes::Rule>) {
        self.rules = rules;
    }
}

pub fn dummy_fighter() -> Fighter {
    Fighter {
        name: "Arches".to_string(),
        stats: Stats {
            health: 10,
            mana: 0,
            attack: 10,
            defense: 0,
            speed: 10,
            wisdom: 0,
            nature: 0,
            demon: 0,
        },
        alive: true,
        rules: Vec::new(),
        team: Team::Ally,
    }
}

pub fn dummy_foe() -> Fighter {
    let mut foe = dummy_fighter();
    foe.stats.speed = 5;
    foe.team = Team::Enemy;
    return foe;
}
