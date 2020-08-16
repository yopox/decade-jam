use crate::fighter::Fighter;

mod equipment;
mod fight;
mod fighter;
mod runes;
mod predefined;

use predefined::rules::AllRules;
use crate::fight::State;

fn main() {
    let mut f1 = fighter::dummy_fighter();
    f1.set_rules(vec![AllRules::Attack2.new()]);
    match fight::Fight::start(vec![f1], vec![fighter::dummy_foe()]) {
        fight::State::AlliesVictory => println!("Allies won!"),
        fight::State::EnemiesVictory => println!("Enemies won :<"),
        State::Draw => println!("Draw!"),
    }
}
