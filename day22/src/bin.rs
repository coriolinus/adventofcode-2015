extern crate day22lib;
use day22lib::{Arena, breadth_first_victory_search, breadth_first_victory_search_with_difficulty};

fn main() {
    let min = breadth_first_victory_search(Arena::default());
    // println!("{}", min.log());
    println!("Min mana required for easy victory: {}", min.mana_spent);

    let min = breadth_first_victory_search_with_difficulty(Arena::default(), true);
    println!("Min mana required for hard victory: {}", min.mana_spent);
}
