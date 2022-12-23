use crate::logic_prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stats {
    health: u16,
    attack: u16,
    defense: u16,
    nature: u16,
    demon: u16,
    speed: u16,
}

impl Stats {
    pub fn new(health: u16, attack: u16, defense: u16, nature: u16, demon: u16, speed: u16) -> Stats {
        Stats { health, attack, defense, nature, demon, speed }
    }

    pub fn reset(&mut self, base: Stats) {
        self.attack = base.attack;
        self.defense = base.defense;
        self.speed = base.speed;
        self.nature = base.nature;
        self.demon = base.demon;
    }

    fn calc(&self, weights: StatWeights) -> u16 {
        let product = (self.attack as i64 * weights.attack as i64 +
            self.defense as i64 * weights.defense as i64 +
            self.speed as i64 * weights.speed as i64 +
            self.nature as i64 * weights.nature as i64 +
            self.demon as i64 * weights.demon as i64) / weights.sum() as i64;
        if product < 0 { return 0; }
        return product as u16;
    }
}

pub struct StatWeights {
    attack: i8,
    defense: i8,
    nature: i8,
    demon: i8,
    speed: i8,
}

impl StatWeights {
    pub fn new(atk: i8, def: i8, nat: i8, dem: i8, spd: i8) -> Self {
        StatWeights {
            attack: atk,
            defense: def,
            nature: nat,
            demon: dem,
            speed: spd,
        }
    }

    pub fn sum(&self) -> u8 {
        let pon = |x: i8| if x < 0 { 0 } else { x as u8 };
        pon(self.attack) + pon(self.defense) + pon(self.speed) + pon(self.nature) + pon(self.demon)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Poisoned,
}

pub struct Fighter {
    name: String,
    base_stats: Stats,
    stats: Stats,
    alive: bool,
    rules: Vec<Rule>,
    default_rule: Rule,
}

impl Fighter {
    pub fn new(name: String, stats: Stats, rules: Vec<Rule>, default_rule: Rule) -> Self {
        Fighter { name, base_stats: stats.clone(), stats: stats.clone(), alive: true, rules, default_rule }
    }

    pub fn turn(&mut self) {
        println!("\tTurn of {} — {}HP", self.name, &self.stats.health);
        self.stats.reset(self.base_stats);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_stat(&self, stat: &Stat) -> u16 {
        match stat {
            Stat::Health => self.stats.health,
            Stat::Attack => self.stats.attack,
            Stat::Defense => self.stats.defense,
            Stat::Speed => self.stats.speed,
            Stat::Nature => self.stats.nature,
            Stat::Demon => self.stats.demon,
        }
    }

    pub fn get_rule(&self, status: &Fight) -> Rule {
        return match self.rules.iter().find(|rule| rule.gate.check(status)) {
            Some(rule) => rule.clone(),
            None => self.default_rule.clone(),
        };
    }

    pub fn set_rules(&mut self, rules: Vec<Rule>) {
        self.rules = rules;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn damage(&mut self, attack: u16, defense: u16, amount: u16) {
        let damage = amount * (attack + 1) / (attack + defense + 1);
        if damage >= self.stats.health {
            println!("\t\t{} lost {}HP…", &self.name, self.stats.health);
            self.stats.health = 0;
            self.alive = false;
            println!("\t\t{} is dead!", &self.name);
        } else {
            self.stats.health -= damage;
            println!("\t\t{} lost {}HP!", &self.name, damage);
        }
    }

    pub fn calc_attack(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.calc(StatWeights::new(4, 0, 0, 0, 0)),
            Element::Natural => self.stats.calc(StatWeights::new(4, 0, 4, -1, 0)),
            Element::Demonic => self.stats.calc(StatWeights::new(4, 0, -1, 4, 0)),
        }
    }

    pub fn calc_defense(&self, element: &Element) -> u16 {
        match element {
            Element::Neutral => self.stats.calc(StatWeights::new(0, 1, 0, 0, 0)),
            Element::Natural => self.stats.calc(StatWeights::new(0, 4, 2, -2, 0)),
            Element::Demonic => self.stats.calc(StatWeights::new(0, 4, -2, 2, 0)),
        }
    }

    pub fn defense(&self) -> Consequence {
        Consequence::Buff {
            stat: Stat::Defense,
            amount: self.base_stats.defense as i32,
            duration: 0,
        }
    }
}
