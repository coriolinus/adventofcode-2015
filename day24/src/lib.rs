//! # Day 24: It Hangs in the Balance
//!
//! It's Christmas Eve, and Santa is loading up the sleigh for this year's deliveries. However,
//! there's one small problem: he can't get the sleigh to balance. If it isn't balanced, he can't
//! defy physics, and nobody gets presents this year.
//!
//! No pressure.
//!
//! Santa has provided you a list of the weights of every package he needs to fit on the sleigh.
//! The packages need to be split into three groups of exactly the same weight, and every package
//! has to fit. The first group goes in the passenger compartment of the sleigh, and the second
//! and third go in containers on either side. Only when all three groups weigh exactly the same
//! amount will the sleigh be able to fly. Defying physics has rules, you know!
//!
//! Of course, that's not the only problem. The first group - the one going in the passenger
//! compartment - needs as few packages as possible so that Santa has some legroom left over.
//! It doesn't matter how many packages are in either of the other two groups, so long as all of
//! the groups weigh the same.
//!
//! Furthermore, Santa tells you, if there are multiple ways to arrange the packages such that the
//! fewest possible are in the first group, you need to choose the way where the first group has
//! the smallest quantum entanglement to reduce the chance of any "complications". The quantum
//! entanglement of a group of packages is the product of their weights, that is, the value you
//! get when you multiply their weights together. Only consider quantum entanglement if the first
//! group has the fewest possible number of packages in it and all groups weigh the same amount.
//!
//! For example, suppose you have ten packages with weights 1 through 5 and 7 through 11. For this
//! situation, some of the unique first groups, their quantum entanglements, and a way to divide
//! the remaining packages are as follows:
//!
//! ```notrust
//! Group 1;             Group 2; Group 3
//! 11 9       (QE= 99); 10 8 2;  7 5 4 3 1
//! 10 9 1     (QE= 90); 11 7 2;  8 5 4 3
//! 10 8 2     (QE=160); 11 9;    7 5 4 3 1
//! 10 7 3     (QE=210); 11 9;    8 5 4 2 1
//! 10 5 4 1   (QE=200); 11 9;    8 7 3 2
//! 10 5 3 2   (QE=300); 11 9;    8 7 4 1
//! 10 4 3 2 1 (QE=240); 11 9;    8 7 5
//! 9 8 3      (QE=216); 11 7 2;  10 5 4 1
//! 9 7 4      (QE=252); 11 8 1;  10 5 3 2
//! 9 5 4 2    (QE=360); 11 8 1;  10 7 3
//! 8 7 5      (QE=280); 11 9;    10 4 3 2 1
//! 8 5 4 3    (QE=480); 11 9;    10 7 2 1
//! 7 5 4 3 1  (QE=420); 11 9;    10 8 2
//! ```
//!
//! Of these, although `10 9 1` has the smallest quantum entanglement (`90`), the configuration
//! with only two packages, `11 9`, in the passenger compartment gives Santa the most legroom and
//! wins. In this situation, the quantum entanglement for the ideal configuration is therefore 99.
//! Had there been two configurations with only two packages in the first group, the one with the
//! smaller quantum entanglement would be chosen.

use std::collections::HashSet;
pub mod summed_subsets;

pub type Package = u8;

#[derive(Clone)]
pub struct Sleigh {
    pub foot: Vec<Package>,
    pub left: Vec<Package>,
    pub right: Vec<Package>,
}

impl Sleigh {
    /// Quantum Entanglement of the footwell of this sleigh.
    pub fn foot_qe(&self) -> u16 {
        self.foot.iter().map(|&x| x as u16).fold(1, |acc, item| acc * item)
    }

    pub fn foot_wt(&self) -> u16 {
        self.foot.iter().map(|&x| x as u16).fold(0, |acc, item| acc + item)
    }

    pub fn left_wt(&self) -> u16 {
        self.left.iter().map(|&x| x as u16).fold(0, |acc, item| acc + item)
    }

    pub fn right_wt(&self) -> u16 {
        self.right.iter().map(|&x| x as u16).fold(0, |acc, item| acc + item)
    }
}

impl Default for Sleigh {
    fn default() -> Sleigh {
        Sleigh {
            foot: Vec::new(),
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

/// Generator of legal sleigh configurations. Main entry point to this module.
///
/// Note: This only handles the case that all of the `Package`s have unique sizes.
pub struct SleighConfigurations {
    packages: Vec<Package>,
    side_wt: Package, // weight for each side
    used_foot: HashSet<Package>,
    used_left: HashSet<Package>,
    prev: Option<Sleigh>,
}

impl Default for SleighConfigurations {
    fn default() -> SleighConfigurations {
        SleighConfigurations {
            packages: Vec::new(),
            side_wt: 0,
            used_foot: HashSet::new(),
            used_left: HashSet::new(),
            prev: None,
        }
    }
}

impl SleighConfigurations {
    /// Construct a new `SleighConfigurations` generator.
    ///
    /// Returns `None` if the total weight can't be evenly divided by 3, or if the biggest package
    /// is bigger than 1/3 of the total weight, because in those circumstances no valid sleigh
    /// configurations can be generated.
    ///
    /// Returns `None` if not all packages are unique, because this solver can't handle that case.
    pub fn new(packages: Vec<Package>) -> Option<SleighConfigurations> {
        let total = packages.iter().fold(0, |acc, item| acc + item);
        if total % 3 != 0 {
            // Invalid configuration; the packages can't be divided into groups of three equal weights
            return None;
        }

        let mut uniques = HashSet::new();
        uniques.extend(packages.iter().cloned());
        if uniques.len() != packages.len() {
            // Invalid because there exist duplicate packages
            return None;
        }

        let mut packages = packages;
        packages.sort();

        if let Some(biggest) = packages.last() {
            if biggest > &(total / 3) {
                // Invalid configuration: the biggest item won't fit into any group
                return None;
            }
        }

        Some(SleighConfigurations {
            packages: packages,
            side_wt: total / 3,
            ..SleighConfigurations::default()
        })
    }

    /// Determine the best sleigh configuration for the given packages.
    ///
    /// The best sleigh configuration is the one for which `sleigh.foot.len()` is minimal. If
    /// multiple sleighs can be configured with equal numbers of items in the footwells, the best
    /// of those is the one for which `sleigh.foot_qe()` is minimal.
    ///
    /// Returns None if the `SleighConfigurations::new()` constructor does for the given packages,
    /// or if no legal configuration can be computed.
    pub fn best(packages: Vec<Package>) -> Option<Sleigh> {
        let sc = SleighConfigurations::new(packages);
        if sc.is_none() {
            return None;
        }
        let mut sc = sc.unwrap();

        let first = sc.next();
        if first.is_none() {
            return None;
        }

        let mut best = first.unwrap();
        for sleigh in sc {
            if sleigh.foot.len() < best.foot.len() {
                best = sleigh;
            } else if sleigh.foot.len() == best.foot.len() && sleigh.foot_qe() < best.foot_qe() {
                best = sleigh;
            }
        }

        Some(best)
    }
}

impl Iterator for SleighConfigurations {
    type Item = Sleigh;

    fn next(&mut self) -> Option<Sleigh> {
        let mut current = Sleigh::default();

        let mut foot_wt_rem = self.side_wt;
        let mut left_wt_rem = self.side_wt;
        // iterate from the top
        for package in self.packages.iter().rev() {
            if *package <= foot_wt_rem {
                foot_wt_rem -= *package;
                current.foot.push(*package);
            } else if *package <= left_wt_rem {
                left_wt_rem -= *package;
                current.left.push(*package);
            } else {
                current.right.push(*package);
            }
        }
        if current.foot_wt() == current.left_wt() && current.foot_wt() == current.right_wt() {
            self.prev = Some(current.clone());
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleigh_example() {
        let sleigh = Sleigh {
            foot: vec![11, 9],
            left: vec![10, 8, 2],
            right: vec![7, 5, 4, 3, 1],
        };

        assert_eq!(sleigh.foot_wt(), sleigh.left_wt());
        assert_eq!(sleigh.foot_wt(), sleigh.right_wt());
        assert_eq!(sleigh.foot_qe(), 99);

        let sleigh = Sleigh {
            foot: vec![10, 9, 1],
            left: vec![11, 7, 2],
            right: vec![8, 5, 4, 3],
        };

        assert_eq!(sleigh.foot_wt(), sleigh.left_wt());
        assert_eq!(sleigh.foot_wt(), sleigh.right_wt());
        assert_eq!(sleigh.foot_qe(), 90);
    }
}
