extern crate util;
use util::get_multiline_input;

extern crate day13lib;
use day13lib::{parse_neighbors, find_best_ordering, evaluate_ordering};

fn main() {
    let lines = get_multiline_input("cat your input file and pipe it here:\n").unwrap();
    let (people, rels) = parse_neighbors(&lines);
    let order = find_best_ordering(&people, &rels);
    let (happiness, _) = evaluate_ordering(&order, &rels);
    println!("Best ordering: {:?}", order);
    println!("Total happiness: {}", happiness);
}
