mod api;
mod logic;
mod predefined;

use logic::prelude::*;
use predefined::prelude::*;

fn main() {
    match Fight::start(vec![Fighters::Arches.new(), Fighters::Arches.new()], vec![Fighters::Bat.new()]) {
        State::AlliesVictory => println!("Allies won!"),
        State::EnemiesVictory => println!("Enemies won :<"),
        State::Draw => println!("Draw!"),
    }
}
