mod equipment;
mod fight;
mod fighter;
mod runes;
mod predefined;

fn main() {
    let fighters = vec![fighter::dummy_fighter(), fighter::dummy_foe()];
    fight::fight(fighters);
}
