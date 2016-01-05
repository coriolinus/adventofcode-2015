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

pub mod summed_subsets;
use summed_subsets::SummedSubsets;

pub type Package = u16;

#[derive(Clone, Debug)]
pub struct Sleigh {
    pub foot: Vec<Package>,
    pub left: Vec<Package>,
    pub right: Vec<Package>,
    pub trunk: Vec<Package>,
}

impl Sleigh {
    /// Quantum Entanglement of the footwell of this sleigh.
    pub fn foot_qe(&self) -> u64 {
        self.foot.iter().map(|&x| x as u64).fold(1, |acc, item| acc * item)
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
            trunk: Vec::new(),
        }
    }
}

/// Generator of legal sleigh configurations. Main entry point to this module.
///
/// Note: This only handles the case that all of the `Package`s have unique sizes.
pub struct SleighConfigurations {
    packages: Vec<Package>,
    side_wt: Package, // weight for each side
    foot_iter: SummedSubsets<Package>,
    use_trunk: bool,
}

impl SleighConfigurations {
    /// Construct a new `SleighConfigurations` generator.
    ///
    /// Returns `None` if the total weight can't be evenly divided by 3, or if the biggest package
    /// is bigger than 1/3 of the total weight, because in those circumstances no valid sleigh
    /// configurations can be generated.
    pub fn new(packages: Vec<Package>, use_trunk: bool) -> Option<SleighConfigurations> {
        let spaces = if use_trunk {4} else {3};
        let total = packages.iter().fold(0, |acc, item| acc + item);
        if total % spaces != 0 {
            // Invalid configuration; the packages can't be divided into groups of three equal weights
            return None;
        }

        let mut packages = packages;
        packages.sort();

        if let Some(biggest) = packages.last() {
            if biggest > &(total / spaces) {
                // Invalid configuration: the biggest item won't fit into any group
                return None;
            }
        }

        Some(SleighConfigurations {
            packages: packages.clone(),
            side_wt: total / spaces,
            foot_iter: SummedSubsets::new(packages, total / spaces),
            use_trunk: use_trunk,
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
    pub fn best(packages: Vec<Package>, use_trunk: bool) -> Option<Sleigh> {
        let sc = SleighConfigurations::new(packages, use_trunk);
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

/// Subtract `subtrahend` from `minuend` as a set, assuming they are both sorted ascending.
fn list_set_subtract<T: PartialEq + Clone>(minuend: &Vec<T>, subtrahend: &Vec<T>) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();
    let mut subt_it = subtrahend.iter().peekable();
    for item in minuend {
        if Some(&item) == subt_it.peek() {
            // it's a match! remove it.
            subt_it.next();
        } else {
            // return this item.
            ret.push(item.to_owned());
        }
    }
    ret
}

impl Iterator for SleighConfigurations {
    type Item = Sleigh;

    fn next(&mut self) -> Option<Sleigh> {
        let next_foot = self.foot_iter.next();
        if next_foot.is_none() {
            return None;
        }
        let next_foot = next_foot.unwrap();
        // we have a unique foot loading, sorted ascending.
        // Our items are also sorted ascending. This is handy.
        // Now, we want to subtract the set of items in the footwell from the rest of our items.
        let side_items = list_set_subtract(&self.packages, &next_foot);
        // now we use this to get just the first remaining subset
        let left_items = SummedSubsets::new(side_items.clone(), self.side_wt).next();
        if left_items.is_none() {
            return None;
        }
        let left_items = left_items.unwrap();

        let right_items;
        let trunk_items;

        if !self.use_trunk {
            right_items = list_set_subtract(&side_items, &left_items);
            trunk_items = Vec::new();
        } else {
            // fill the trunk also
            let right_rear_items = list_set_subtract(&side_items, &left_items);
            let right_maybe = SummedSubsets::new(right_rear_items.clone(), self.side_wt).next();
            if right_maybe.is_none() {
                return None;
            }
            right_items = right_maybe.unwrap();
            trunk_items = list_set_subtract(&right_rear_items, &right_items);
        }

        Some(Sleigh {
            foot: next_foot,
            left: left_items,
            right: right_items,
            trunk: trunk_items,
        })
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
            trunk: vec![],
        };

        assert_eq!(sleigh.foot_wt(), sleigh.left_wt());
        assert_eq!(sleigh.foot_wt(), sleigh.right_wt());
        assert_eq!(sleigh.foot_qe(), 99);

        let sleigh = Sleigh {
            foot: vec![10, 9, 1],
            left: vec![11, 7, 2],
            right: vec![8, 5, 4, 3],
            trunk: vec![],
        };

        assert_eq!(sleigh.foot_wt(), sleigh.left_wt());
        assert_eq!(sleigh.foot_wt(), sleigh.right_wt());
        assert_eq!(sleigh.foot_qe(), 90);
    }

    #[test]
    fn test_example() {
        let items = vec![1,2,3,4,5,7,8,9,10,11];
        let best = SleighConfigurations::best(items, false).unwrap();
        println!("Best sleigh configuration: {:?}", best);
        assert_eq!(best.foot_qe(), 99);
    }

    #[test]
    fn test_example_with_trunk() {
        let items = vec![1,2,3,4,5,7,8,9,10,11];
        let best = SleighConfigurations::best(items, true).unwrap();
        println!("Best sleigh configuration: {:?}", best);
        assert_eq!(best.foot_qe(), 44);
    }
}
