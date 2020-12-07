//! # Day 13: Knights of the Dinner Table
//!
//! In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along!
//! This year, you resolve, will be different. You're going to find the optimal seating arrangement
//! and avoid all those awkward conversations.
//!
//! You start by writing up a list of everyone invited and the amount their happiness would
//! increase or decrease if they were to find themselves sitting next to each other person. You
//! have a circular table that will be just big enough to fit everyone comfortably, and so each
//! person will have exactly two neighbors.

use std::collections::{HashMap, HashSet};

use permutohedron::heap_recursive;

use util::parse::Parser;

pub type Relationships = HashMap<(String, String), i32>;

pub fn parse_neighbors(lines: &str) -> (HashSet<String>, Relationships) {
    // create parser
    let parser = Parser::default()
        .force_lowercase(false)
        .require_at_least(Some(11))
        .require_fewer_than(Some(12))
        .fixed_tokens({
            let mut h = HashMap::new();
            h.insert(1, "would".to_string());
            h.insert(4, "happiness".to_string());
            h.insert(5, "units".to_string());
            h.insert(6, "by".to_string());
            h.insert(7, "sitting".to_string());
            h.insert(8, "next".to_string());
            h.insert(9, "to".to_string());
            h
        });

    let mut r = Relationships::default();
    let mut p = HashSet::default();

    for line in lines.split('\n') {
        let mut line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        // Discard the trailing dot.
        line.pop();

        if let Ok(v) = parser.parse(&line) {
            let ref left = v.tokens[0];
            let ref gain_lose = v.tokens[1];
            let ref n_str = v.tokens[2];
            let ref right = v.tokens[3];

            let mut n = i32::from_str_radix(n_str, 10).unwrap();

            if gain_lose == &String::from("lose") {
                n *= -1;
            } else if gain_lose == &String::from("gain") {
            } else {
                continue;
            }

            r.insert((left.clone(), right.clone()), n);
            p.insert(left.clone());
        }
    }

    (p, r)
}

pub fn evaluate_ordering(
    people: &Vec<String>,
    rels: &Relationships,
) -> (i32, HashMap<String, i32>) {
    let mut total_happiness = 0;
    let mut personal_happiness = HashMap::new();

    // compute personal happiness for each member of the circle
    for (i, person) in people.iter().enumerate() {
        let ref left = people[if i > 0 { i - 1 } else { people.len() - 1 }];
        let ref right = people[if i < people.len() - 1 { i + 1 } else { 0 }];

        let ph = rels.get(&(person.to_owned(), left.to_owned())).unwrap()
            + rels.get(&(person.to_owned(), right.to_owned())).unwrap();

        total_happiness += ph;
        personal_happiness.insert(person.clone(), ph);
    }

    (total_happiness, personal_happiness)
}

pub fn find_best_ordering(people: &HashSet<String>, rels: &Relationships) -> Vec<String> {
    let mut places = people.iter().collect::<Vec<_>>();
    let mut ret = Vec::new();

    let mut cur_happiness = i32::min_value();

    heap_recursive(&mut places, |ordering| {
        // ordering = [&String]
        let this_ord: Vec<_> = ordering.to_vec().iter().map(|&s| s.clone()).collect();
        let (this_happiness, _) = evaluate_ordering(&this_ord, &rels);

        if this_happiness > cur_happiness {
            ret = this_ord;
            cur_happiness = this_happiness;
        }
    });

    ret
}
