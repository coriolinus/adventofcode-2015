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

use aoc2015::parse;
use permutohedron::heap_recursive;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    path::Path,
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, parse_display::FromStr, parse_display::Display)]
#[display(style = "snake_case")]
enum Modify {
    Gain,
    Lose,
}

impl Modify {
    fn modify(self, n: i32) -> i32 {
        match self {
            Self::Gain => n,
            Self::Lose => -n,
        }
    }
}

#[derive(Clone, Debug, parse_display::FromStr, parse_display::Display)]
#[display("{who} would {modify} {qty} happiness units by sitting next to {other}.")]
struct Edge {
    who: String,
    modify: Modify,
    qty: i32,
    other: String,
}

pub type Person = usize;
pub type Relationships = HashMap<(Person, Person), i32>;

struct Graph {
    relationships: Relationships,
    index: Vec<String>,
}

impl FromIterator<Edge> for Graph {
    fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min_size, _) = iter.size_hint();

        // create temporary structures holding string data
        let mut people = HashSet::with_capacity(min_size);
        let mut relationships = HashMap::with_capacity(min_size);

        for Edge {
            who,
            modify,
            qty,
            other,
        } in iter
        {
            people.insert(who.clone());
            people.insert(other.clone());
            relationships.insert((who, other), modify.modify(qty));
        }

        // convert those data structures into ones which are easier to use, refering to people
        // by their positional index in `index`.
        let index = {
            let mut index: Vec<_> = people.iter().cloned().collect();
            index.sort();
            index
        };

        let index_of = {
            let mut index_of = HashMap::with_capacity(index.len());
            for (idx, person) in index.iter().cloned().enumerate() {
                index_of.insert(person, idx);
            }
            index_of
        };

        let relationships = relationships
            .into_iter()
            .map(|((who, other), qty)| ((index_of[&who], index_of[&other]), qty))
            .collect();

        Graph {
            relationships,
            index,
        }
    }
}

pub fn evaluate_ordering(ordering: &[Person], relationships: &Relationships) -> i32 {
    let mut total_happiness = 0;

    // compute personal happiness for each member of the circle
    for (i, person) in ordering.iter().copied().enumerate() {
        let left = ordering[if i > 0 { i - 1 } else { ordering.len() - 1 }];
        let right = ordering[if i < ordering.len() - 1 { i + 1 } else { 0 }];

        total_happiness += relationships
            .get(&(person, left))
            .copied()
            .unwrap_or_default()
            + relationships
                .get(&(person, right))
                .copied()
                .unwrap_or_default();
    }

    total_happiness
}

pub fn find_best_ordering(n_people: usize, relationships: &Relationships) -> Vec<Person> {
    let mut ordering: Vec<_> = (0..n_people).collect();
    let mut best_ordering = Vec::new();
    let mut cur_happiness = i32::MIN;

    heap_recursive(&mut ordering, |ordering| {
        let this_happiness = evaluate_ordering(ordering, &relationships);

        if this_happiness > cur_happiness {
            cur_happiness = this_happiness;
            best_ordering = ordering.to_vec();
        }
    });

    best_ordering
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let Graph {
        relationships,
        index,
    } = parse(input)?.collect();

    let n_people = index.len();
    let best_ordering = find_best_ordering(n_people, &relationships);
    let happiness = evaluate_ordering(&best_ordering, &relationships);
    println!("Best happiness: {}", happiness);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let Graph {
        relationships,
        index,
    } = parse(input)?.collect();

    let n_people = index.len() + 1;
    let best_ordering = find_best_ordering(n_people, &relationships);
    let happiness = evaluate_ordering(&best_ordering, &relationships);

    println!("Best happiness (+1 guest): {}", happiness);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
