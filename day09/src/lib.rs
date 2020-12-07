//! # Day 9: All in a Single Night
//!
//! Every year, Santa manages to deliver all of his presents in a single night.
//!
//! This year, however, he has some new locations to visit; his elves have provided him the
//! distances between every pair of locations. He can start and end at any two (different)
//! locations he wants, but he must visit each location exactly once. What is the shortest distance
//! he can travel to achieve this?

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use util::parse::Parser;

use permutohedron::heap_recursive;

// we're going to do this the quick, dumb way.
type DistMap = HashMap<(String, String), usize>;

pub struct Routes {
    dist_map: DistMap,
    places: HashSet<String>,
}

impl Routes {
    fn new() -> Routes {
        Routes {
            dist_map: DistMap::new(),
            places: HashSet::new(),
        }
    }

    pub fn parse_routes(lines: &str) -> Routes {
        // create parser
        let parser = Parser::default()
            .force_lowercase(false)
            .require_at_least(Some(5))
            .require_fewer_than(Some(6))
            .fixed_tokens({
                let mut h = HashMap::new();
                h.insert(1, "to".to_string());
                h.insert(3, "=".to_string());
                h
            });

        let mut o = Routes::new();

        for line in lines.split('\n') {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Ok(v) = parser.parse(line) {
                let ref from = v.tokens[0];
                let ref to = v.tokens[1];
                let ref dist = v.tokens[2];

                if let Ok(dist) = usize::from_str_radix(&dist, 10) {
                    o.add(from.clone(), to.clone(), dist);
                }
            }
        }

        o
    }

    pub fn add(&mut self, from: String, to: String, dist: usize) {
        self.dist_map.insert((from.clone(), to.clone()), dist);
        self.dist_map.insert((to.clone(), from.clone()), dist);
        self.places.insert(from.clone());
        self.places.insert(to.clone());
    }

    pub fn find_extreme(&self, order: Ordering, default_dist: usize) -> Route {
        let mut places = self.places.iter().collect::<Vec<_>>();

        let mut ret = Route {
            stops: Vec::new(),
            dist: default_dist,
        };
        heap_recursive(&mut places, |ordering| {
            // ordering = [&String]
            let this_dist: usize = ordering
                .windows(2)
                .map(|window| (window[0], window[1]))
                .map(|(from, to)| self.dist_map.get(&(from.clone(), to.clone())).unwrap())
                .fold(0, std::ops::Add::add);

            if this_dist.cmp(&ret.dist) == order {
                ret.stops = ordering.iter().map(|&s| s.clone()).collect::<Vec<_>>();
                ret.dist = this_dist;
            }
        });

        ret
    }

    pub fn find_shortest(&self) -> Route {
        self.find_extreme(Ordering::Less, usize::max_value())
    }

    pub fn find_longest(&self) -> Route {
        self.find_extreme(Ordering::Greater, 0)
    }
}

#[derive(Debug)]
pub struct Route {
    pub stops: Vec<String>,
    pub dist: usize,
}

#[cfg(test)]
mod test {
    use super::Routes;

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
        let mut expected_route = vec!["London", "Dublin", "Belfast"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let routes = Routes::parse_routes(lines);
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
        let mut expected_route = vec!["Dublin", "London", "Belfast"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let routes = Routes::parse_routes(lines);
        let longest = routes.find_longest();

        println!("Longest route: {:?}", longest);

        assert_eq!(982, longest.dist);
        let fwd = longest.stops == expected_route;
        expected_route.reverse();
        let rev = longest.stops == expected_route;

        assert!(fwd || rev);
    }
}
