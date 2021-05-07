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

use std::{
    cmp::Reverse,
    convert::{TryFrom, TryInto},
    path::Path,
};

mod summed_subsets;

pub type Package = u16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compartment {
    Footwell,
    LeftSaddle,
    RightSaddle,
    Trunk,
}

impl Compartment {
    /// Use the next compartment.
    ///
    /// Returns `true` when rollover to the default.
    fn next(&mut self, use_trunk: bool) -> bool {
        *self = match self {
            Compartment::Footwell => Compartment::LeftSaddle,
            Compartment::LeftSaddle => Compartment::RightSaddle,
            Compartment::RightSaddle => {
                if use_trunk {
                    Compartment::Trunk
                } else {
                    Compartment::Footwell
                }
            }
            Compartment::Trunk => Compartment::Footwell,
        };
        *self == Compartment::Footwell
    }
}

impl Default for Compartment {
    fn default() -> Self {
        Compartment::Footwell
    }
}

impl TryFrom<usize> for Compartment {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Compartment::Footwell),
            1 => Ok(Compartment::LeftSaddle),
            2 => Ok(Compartment::RightSaddle),
            3 => Ok(Compartment::Trunk),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PackingList<'a> {
    configurator: &'a Configurator,
    compartments: Vec<Compartment>,
}

impl<'a> PackingList<'a> {
    fn packages_in(&self, want: Compartment) -> impl '_ + Iterator<Item = Package> {
        self.configurator
            .packages
            .iter()
            .zip(self.compartments.iter())
            .filter_map(move |(package, present_in)| (*present_in == want).then(|| *package))
    }

    /// Quantum Entanglement of the footwell of this sleigh.
    pub fn qe(&self, compartment: Compartment) -> u64 {
        self.packages_in(compartment)
            .map(|weight| weight as u64)
            .product()
    }

    pub fn weight(&self, compartment: Compartment) -> Package {
        self.packages_in(compartment).sum()
    }
}

/// Generator of legal sleigh configurations. Main entry point to this module.
///
/// Note: This only handles the case that all of the `Package`s have unique sizes.
#[derive(Debug)]
pub struct Configurator {
    // always reverse-sorted
    packages: Vec<Package>,
    side_weight: Package, // weight for each side
    use_trunk: bool,
}

impl Configurator {
    /// Construct a new `SleighConfigurations` generator.
    ///
    /// Returns `None` if the total weight can't be evenly divided by 3, or if the biggest package
    /// is bigger than 1/3 of the total weight, because in those circumstances no valid sleigh
    /// configurations can be generated.
    pub fn new(mut packages: Vec<Package>, use_trunk: bool) -> Option<Configurator> {
        let spaces = if use_trunk { 4 } else { 3 };
        let total: Package = packages.iter().sum();
        if total % spaces != 0 {
            // Invalid configuration; the packages can't be divided into groups of three equal weights
            return None;
        }
        let side_weight = total / spaces;

        packages.sort_unstable_by_key(|weight| Reverse(*weight));

        if *packages.first()? > side_weight {
            // Invalid configuration: the biggest item won't fit into any group
            return None;
        }

        Some(Configurator {
            packages,
            side_weight,
            use_trunk,
        })
    }

    fn packing_list<'a>(&'a self, compartments: Vec<Compartment>) -> PackingList<'a> {
        PackingList {
            configurator: &self,
            compartments,
        }
    }

    /// Find an initial configuration satisfying the balance requirements.
    fn fill_compartments(&self) -> Option<Vec<Compartment>> {
        let mut compartments = vec![Compartment::default(); self.packages.len()];
        let mut compartment_weights = [0; 4];
        let high_idx = if self.use_trunk { 4 } else { 3 };
        'outer: for (pkg_idx, package) in self.packages.iter().enumerate() {
            for cpt_idx in 0..high_idx {
                if compartment_weights[cpt_idx] + package <= self.side_weight {
                    compartment_weights[cpt_idx] += package;
                    compartments[pkg_idx] = cpt_idx.try_into().ok()?;
                    continue 'outer;
                }
            }
            // could not find a compartment into which this package could fit
            return None;
        }
        // we have generated an appropriate packing
        compartment_weights[..high_idx]
            .iter()
            .all(|weight| *weight == self.side_weight)
            .then(move || compartments)
    }

    /// Generate a series of packing lists satisfying the balance requirements.
    fn generate_packing_lists<'a>(&'a self) -> impl Iterator<Item = PackingList<'a>> {
        let use_trunk = self.use_trunk;
        let side_weight = self.side_weight;
        std::iter::successors(self.fill_compartments(), move |prev| {
            // the task here is to generate the next permutation of elements among the groups
            // such that the constant-sum property is respected. We can (probably) use a variation
            // of the next-lexicographic-permutation algorithm to generate this efficiently.
            // see https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
            let mut next = prev.clone();
            unimplemented!()
        })
        .map(move |compartments| self.packing_list(compartments))
    }

    /// Determine the best sleigh configuration for the given packages.
    ///
    /// The best sleigh configuration is the one for which `sleigh.foot.len()` is minimal. If
    /// multiple sleighs can be configured with equal numbers of items in the footwells, the best
    /// of those is the one for which `sleigh.foot_qe()` is minimal.
    ///
    /// Returns None if the `SleighConfigurations::new()` constructor does for the given packages,
    /// or if no legal configuration can be computed.
    pub fn best<'a>(&'a self) -> Option<PackingList<'a>> {
        unimplemented!()
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let packages: Vec<Package> = aoclib::parse(input)?.collect();
    let trunk = false;
    let configurator =
        Configurator::new(packages, trunk).ok_or(Error::NoAppropriateLoading(trunk))?;
    let best = configurator
        .best()
        .ok_or(Error::NoAppropriateLoading(trunk))?;
    println!(
        "QE of best enganglement (no trunk):   {:12}",
        best.qe(Compartment::Footwell)
    );
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let packages: Vec<Package> = aoclib::parse(input)?.collect();
    let trunk = true;
    let configurator =
        Configurator::new(packages, trunk).ok_or(Error::NoAppropriateLoading(trunk))?;
    let best = configurator
        .best()
        .ok_or(Error::NoAppropriateLoading(trunk))?;
    println!(
        "QE of best enganglement (with trunk): {:12}",
        best.qe(Compartment::Footwell)
    );
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to find an appropriate loading (trunk: {0})")]
    NoAppropriateLoading(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compartments_from_groups(
        footwell: &[Package],
        left_saddle: &[Package],
        right_saddle: &[Package],
        trunk: &[Package],
    ) -> Vec<Compartment> {
        let mut compartments = Vec::with_capacity(
            footwell.len() + left_saddle.len() + right_saddle.len() + trunk.len(),
        );

        let mut groups = [
            footwell.to_vec(),
            left_saddle.to_vec(),
            right_saddle.to_vec(),
            trunk.to_vec(),
        ];
        groups.iter_mut().for_each(|group| group.sort_unstable());

        while let Some(idx_of_highest) = groups
            .iter()
            .map(|group| group.last())
            .enumerate()
            .filter_map(|(idx, maybe_top)| maybe_top.map(|top| (top, idx)))
            .max()
            .map(|(_, idx)| idx)
        {
            groups[idx_of_highest].pop();
            compartments.push(idx_of_highest.try_into().unwrap());
        }

        compartments
    }

    #[test]
    fn test_compartments_from_groups() {
        assert_eq!(
            compartments_from_groups(&[11, 9], &[10, 8, 2], &[7, 5, 4, 3, 1], &[]),
            vec![
                Compartment::Footwell,    // 11
                Compartment::LeftSaddle,  // 10
                Compartment::Footwell,    // 9
                Compartment::LeftSaddle,  // 8
                Compartment::RightSaddle, // 7
                Compartment::RightSaddle, // 5
                Compartment::RightSaddle, // 4
                Compartment::RightSaddle, // 3
                Compartment::LeftSaddle,  // 2
                Compartment::RightSaddle, // 1
            ]
        );
    }

    #[test]
    fn test_fill_compartments() {
        let packages: Vec<Package> = (1..=5).chain(7..=11).collect();
        let configurator = Configurator::new(packages, false).unwrap();
        assert_eq!(
            configurator.fill_compartments().unwrap(),
            compartments_from_groups(&[11, 9], &[10, 8, 2], &[7, 5, 4, 3, 1], &[])
        );
    }

    #[test]
    fn test_sleigh_example() {
        let configurator = Configurator::new((1..=5).chain(7..=11).collect(), false).unwrap();
        let packing_list = PackingList {
            configurator: &configurator,
            compartments: compartments_from_groups(&[11, 9], &[10, 8, 2], &[7, 5, 4, 3, 1], &[]),
        };

        assert_eq!(
            packing_list.weight(Compartment::Footwell),
            packing_list.weight(Compartment::LeftSaddle)
        );
        assert_eq!(
            packing_list.weight(Compartment::Footwell),
            packing_list.weight(Compartment::RightSaddle)
        );
        assert_eq!(packing_list.qe(Compartment::Footwell), 99);

        let packing_list = PackingList {
            configurator: &configurator,
            compartments: compartments_from_groups(&[10, 9, 1], &[11, 7, 2], &[8, 5, 4, 3], &[]),
        };

        assert_eq!(
            packing_list.weight(Compartment::Footwell),
            packing_list.weight(Compartment::LeftSaddle)
        );
        assert_eq!(
            packing_list.weight(Compartment::Footwell),
            packing_list.weight(Compartment::RightSaddle)
        );
        assert_eq!(packing_list.qe(Compartment::Footwell), 90);
    }

    #[test]
    fn test_example() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let configurator = Configurator::new(items, false).unwrap();
        let best = configurator.best().unwrap();
        println!("Best sleigh configuration: {:?}", best);
        assert_eq!(best.qe(Compartment::Footwell), 99);
    }

    #[test]
    fn test_example_with_trunk() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let configurator = Configurator::new(items, true).unwrap();
        let best = configurator.best().unwrap();
        println!("Best sleigh configuration: {:?}", best);
        assert_eq!(best.qe(Compartment::Footwell), 44);
    }
}
