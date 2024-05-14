use crate::logic::prelude::*;
use crate::predefined::prelude::*;
use crate::predefined::prelude::Swords::WoodenSword;

pub enum Fighters {
    Arches,
    Bat,
}

impl Fighters {
    pub fn new(self) -> Fighter {
        get(self)
    }
}

fn get(fighter: Fighters) -> Fighter {
    match fighter {
        Fighters::Arches => Fighter::new(
            "Arches".to_string(),
            Stats::new(100, 5, 10, 10, 0, 10),
            vec![Rules::Attack2.new()],
            Some(Box::new(WoodenSword.new())),
        ),
        Fighters::Bat => Fighter::new(
            "Bat".to_string(),
            Stats::new(60, 8, 15, 5, 8, 4),
            vec![Rules::Attack2.new()],
            Some(Box::new(WoodenSword.new())),
        )
    }
}
