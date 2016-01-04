extern crate day22lib;
use day22lib::{Arena, breadth_first_victory_search};

fn main() {
    let min = breadth_first_victory_search(Arena::default());
    println!("{}", min.log());
    println!("Min mana required for victory: {}", min.mana_spent);
}
