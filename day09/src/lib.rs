//! # Day 9: All in a Single Night
//!
//! Every year, Santa manages to deliver all of his presents in a single night.
//!
//! This year, however, he has some new locations to visit; his elves have provided him the
//! distances between every pair of locations. He can start and end at any two (different)
//! locations he wants, but he must visit each location exactly once. What is the shortest distance
//! he can travel to achieve this?

use aoc2015::parse;
use permutohedron::heap_recursive;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
pub struct Route {
    pub stops: Vec<String>,
    pub dist: u32,
}

#[derive(Clone, Debug, parse_display::FromStr, parse_display::Display)]
#[display("{from} to {to} = {distance}")]
struct Edge {
    from: String,
    to: String,
    distance: u32,
}

// we're going to do this the quick, dumb way.
type DistMap = HashMap<(String, String), u32>;

pub struct Routes {
    dist_map: DistMap,
    places: HashSet<String>,
}

impl std::iter::FromIterator<Edge> for Routes {
    fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min_contents, _) = iter.size_hint();

        let mut dist_map = DistMap::with_capacity(min_contents);
        let mut places = HashSet::with_capacity(min_contents);

        for Edge { from, to, distance } in iter {
            dist_map.insert((from.clone(), to.clone()), distance);
            dist_map.insert((to.clone(), from.clone()), distance);
            places.insert(from);
            places.insert(to);
        }

        Routes { dist_map, places }
    }
}

impl Routes {
    fn find_extreme(&self, order: Ordering, default_dist: u32) -> Route {
        let mut places: Vec<_> = self.places.iter().collect();

        let mut route = Route {
            stops: Vec::new(),
            dist: default_dist,
        };

        heap_recursive(&mut places, |ordering| {
            let this_dist: u32 = ordering
                .windows(2)
                .map(|window| {
                    let from = window[0];
                    let to = window[1];
                    self.dist_map.get(&(from.clone(), to.clone())).unwrap()
                })
                .sum();

            if this_dist.cmp(&route.dist) == order {
                route.stops = ordering.iter().map(|&s| s.clone()).collect::<Vec<_>>();
                route.dist = this_dist;
            }
        });

        route
    }

    pub fn find_shortest(&self) -> Route {
        self.find_extreme(Ordering::Less, !0)
    }

    pub fn find_longest(&self) -> Route {
        self.find_extreme(Ordering::Greater, 0)
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let routes: Routes = parse(input)?.collect();
    let shortest = routes.find_shortest();
    println!("shortest route length: {}", shortest.dist);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let routes: Routes = parse(input)?.collect();
    let longest = routes.find_longest();
    println!("longest route length: {}", longest.dist);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    /// For example, given the following distances:
    ///
    /// London to Dublin = 464
    /// London to Belfast = 518
    /// Dublin to Belfast = 141
    /// The possible routes are therefore:

    /// Dublin -> London -> Belfast = 982
    /// London -> Dublin -> Belfast = 605
    /// London -> Belfast -> Dublin = 659
    /// Dublin -> Belfast -> London = 659
    /// Belfast -> Dublin -> London = 605
    /// Belfast -> London -> Dublin = 982
    /// The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.
    ///
    /// What is the distance of the shortest route?
    #[test]
    fn test_example_shortest() {
        let lines = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let mut expected_route = ["London", "Dublin", "Belfast"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let routes: Routes = lines
            .split('\n')
            .map(|line| line.parse::<Edge>().unwrap())
            .collect();
        let shortest = routes.find_shortest();

        println!("Shortest route: {:?}", shortest);

        assert_eq!(605, shortest.dist);
        let fwd = shortest.stops == expected_route;
        expected_route.reverse();
        let rev = shortest.stops == expected_route;

        assert!(fwd || rev);
    }

    /// The next year, just to show off, Santa decides to take the route with the longest distance
    /// instead.
    ///
    /// He can still start and end at any two (different) locations he wants, and he still must
    /// visit each location exactly once.
    ///
    /// For example, given the distances above, the longest route would be `982` via (for example)
    /// `Dublin -> London -> Belfast`.
    #[test]
    fn test_example_longest() {
        let lines = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        let mut expected_route = ["Dublin", "London", "Belfast"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let routes: Routes = lines
            .split('\n')
            .map(|line| line.parse::<Edge>().unwrap())
            .collect();
        let longest = routes.find_longest();

        println!("Longest route: {:?}", longest);

        assert_eq!(982, longest.dist);
        let fwd = longest.stops == expected_route;
        expected_route.reverse();
        let rev = longest.stops == expected_route;

        assert!(fwd || rev);
    }
}
