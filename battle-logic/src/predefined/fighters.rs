use crate::logic_prelude::*;
use crate::predefined_prelude::*;

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
            Stats::new(100, 20, 10, 10, 10, 10, 0),
            vec![Rules::Attack2.new()],
            Rules::Wait.new()
        ),
        Fighters::Bat => Fighter::new(
            "Bat".to_string(),
            Stats::new(30, 8, 10, 5, 8, 4, 8),
            vec![Rules::Attack2.new()],
            Rules::Wait.new()
        )
    }
}
