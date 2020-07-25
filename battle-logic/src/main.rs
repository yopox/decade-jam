mod fight;
mod fighter;
mod runes;

fn main() {
    println!("{:?}", &runes::predefined::CAREFUL.get_action());
}
