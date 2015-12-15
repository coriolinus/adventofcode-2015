extern crate util;
use util::get_multiline_input;

extern crate day13lib;
use day13lib::{parse_neighbors, find_best_ordering, evaluate_ordering};

fn main() {
    let lines = get_multiline_input("cat your input file and pipe it here:\n").unwrap();
    let (mut people, mut rels) = parse_neighbors(&lines);
    let order = find_best_ordering(&people, &rels);
    let (happiness, _) = evaluate_ordering(&order, &rels);
    println!("Best ordering: {:?}", order);
    println!("Total happiness: {}", happiness);

    for person in &people {
        rels.insert((person.clone(), "Self".to_string()), 0);
        rels.insert(("Self".to_string(), person.clone()), 0);
    }
    people.insert("Self".to_string());

    let order = find_best_ordering(&people, &rels);
    let (happiness, _) = evaluate_ordering(&order, &rels);
    println!("Best with-self ordering: {:?}", order);
    println!("Total with-self happiness: {}", happiness);
}