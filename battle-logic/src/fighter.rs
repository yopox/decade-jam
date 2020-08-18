use crate::equipment::{Element, Spell, Weapon};
use crate::predefined;
use crate::predefined::rules::AllRules;
use crate::predefined::spells::AllSpells;
use crate::predefined::weapons::AllWeapons;
use crate::runes::{Action, Rule, Stat};
use crate::{fight, runes};

struct Stats {
    health: u16,
    attack: u16,
    defense: u16,
    wisdom: u16,
    speed: u16,
    nature: u16,
    demon: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Poisoned,
}

pub struct Fighter {
    name: String,
    stats: Stats,
    alive: bool,
    rules: Vec<runes::Rule>,
    default_rule: Rule,
}

pub enum DamageType {
    NEUTRAL,
    NATURE,
    DEMON,
}

impl Fighter {
    pub fn turn(&mut self, status: &mut fight::Fight) {
        println!("Turn of {}.", self.name)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_stat(&self, stat: &runes::Stat) -> u16 {
        match stat {
            Stat::Health => self.stats.health,
            Stat::Attack => self.stats.attack,
            Stat::Defense => self.stats.defense,
            Stat::Wisdom => self.stats.wisdom,
            Stat::Speed => self.stats.speed,
            Stat::Nature => self.stats.nature,
            Stat::Demon => self.stats.demon,
        }
    }

    pub fn get_rule(&self, status: &fight::Fight) -> Rule {
        return match self.rules.iter().find(|rule| rule.check(status)) {
            Some(rule) => rule.clone(),
            None => self.default_rule.clone(),
        };
    }

    pub fn set_rules(&mut self, rules: Vec<runes::Rule>) {
        self.rules = rules;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn get_elemental_physical_attack(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.attack,
            Element::Demonic => (self.stats.attack + self.stats.demon) / 2,
            Element::Natural => (self.stats.attack + self.stats.nature) / 2,
        }
    }

    pub fn get_elemental_physical_defense(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.defense,
            Element::Demonic => {
                (self.stats.defense + diff(self.stats.demon, self.stats.nature) * 2) / 3
            }
            Element::Natural => {
                (self.stats.defense + diff(self.stats.nature, self.stats.demon) * 2) / 3
            }
        }
    }

    pub fn get_elemental_magical_attack(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.wisdom,
            Element::Demonic => (self.stats.wisdom + self.stats.demon) / 2,
            Element::Natural => (self.stats.wisdom + self.stats.nature) / 2,
        }
    }

    pub fn get_elemental_magical_defense(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.defense,
            Element::Demonic => {
                (self.stats.defense + diff(self.stats.demon, self.stats.nature) * 2) / 3
            }
            Element::Natural => {
                (self.stats.defense + diff(self.stats.nature, self.stats.demon) * 2) / 3
            }
        }
    }

    pub(crate) fn damage(&mut self, attack: u16, defense: u16) {
        let damage = attack * (1 + 3 * (attack + 1) / (attack + defense + 1)) / 4;
        if damage > self.stats.health {
            self.stats.health = 0;
            self.alive = false
        } else {
            self.stats.health -= damage;
        }
    }
}

pub fn dummy_fighter() -> Fighter {
    Fighter {
        name: "Arches".to_string(),
        stats: Stats {
            health: 40,
            attack: 10,
            defense: 5,
            speed: 10,
            wisdom: 0,
            nature: 0,
            demon: 0,
        },
        alive: true,
        rules: Vec::new(),
        default_rule: AllRules::Default.new(),
    }
}

pub fn dummy_foe() -> Fighter {
    let mut foe = dummy_fighter();
    foe.name = "Azazel".to_string();
    foe.stats.speed = 5;
    return foe;
}

fn diff(x: u16, y: u16) -> u16 {
    match x < y {
        true => 0,
        false => x - y,
    }
}
