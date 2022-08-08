mod modules;
use crate::modules::attributes::Attributes;

fn main() {
    let my_attributes = Attributes::new_random();

    eprintln!("{}", my_attributes);
}
