use logic_prelude::*;
use predefined_prelude::*;

pub mod logic {
    pub mod equipment;
    pub mod fight;
    pub mod fighter;
    pub mod rune;
}

pub mod predefined {
    pub mod fighters;
    pub mod rules;
    pub mod weapons;
    pub mod effects;
}

pub mod logic_prelude {
    pub use crate::logic::equipment::*;
    pub use crate::logic::fight::*;
    pub use crate::logic::fighter::*;
    pub use crate::logic::rune::*;
}

pub mod predefined_prelude {
    pub use crate::predefined::fighters::*;
    pub use crate::predefined::rules::*;
    pub use crate::predefined::weapons::*;
    pub use crate::predefined::effects::*;
}

fn main() {
    match Fight::start(vec![Fighters::Arches.new()], vec![Fighters::Bat.new()]) {
        State::AlliesVictory => println!("Allies won!"),
        State::EnemiesVictory => println!("Enemies won :<"),
        State::Draw => println!("Draw!"),
    }
}
