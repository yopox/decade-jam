use crate::fighter::Fighter;

mod equipment;
mod fight;
mod fighter;
mod runes;
mod predefined;

fn main() {
    fight::Fight::start(vec![fighter::dummy_fighter()], vec![fighter::dummy_foe()]);
}
