use crate::{fight, runes};
use crate::runes::{Action, Rule, Stat};
use crate::predefined;
use crate::predefined::rules::AllRules;
use crate::predefined::weapons::AllWeapons;
use crate::predefined::spells::AllSpells;

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

#[derive(Debug, PartialEq)]
pub enum Status {
    Poisoned,
}

pub struct Fighter {
    name: String,
    stats: Stats,
    pub alive: bool,
    rules: Vec<runes::Rule>,
    team: Team,
    default_rule: Rule
}

pub enum DamageType {
    NEUTRAL,
    NATURE,
    DEMON
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
            None => &self.default_rule,
        };
    }

    pub fn set_rules(&mut self, rules: Vec<runes::Rule>) {
        self.rules = rules;
    }

    pub fn perform(&self, action: &runes::Action, status: &fight::Status) {
        match action {
            runes::Action::Attack(weapon, target) => {}
            Action::Defense => {}
            Action::Spell(spell, target) => {}
            Action::Wait => {
                println!("{} waits.", &self.name);
            }
        };
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
        default_rule: AllRules::Default.new()
    }
}

pub fn dummy_foe() -> Fighter {
    let mut foe = dummy_fighter();
    foe.name = "Azazel".to_string();
    foe.stats.speed = 5;
    foe.team = Team::Enemy;
    return foe;
}
