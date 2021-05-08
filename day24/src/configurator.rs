use std::{cmp::Reverse, collections::HashSet, rc::Rc};

use crate::{
    packing_list::compartments_from_groups, BoundedPermutationGenerator, Compartment, Package,
    PackingList,
};

type Group3 = (Rc<Vec<Package>>, Rc<Vec<Package>>, Rc<Vec<Package>>);
type Group4 = (
    Rc<Vec<Package>>,
    Rc<Vec<Package>>,
    Rc<Vec<Package>>,
    Rc<Vec<Package>>,
);

/// Generator of legal sleigh configurations. Main entry point to this module.
///
/// Note: This only handles the case that all of the `Package`s have unique sizes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Configurator {
    // always reverse-sorted
    pub(crate) packages: Rc<Vec<Package>>,
    pub(crate) side_weight: Package, // weight for each side
    pub(crate) use_trunk: bool,
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

        packages.sort_unstable_by_key(|package| Reverse(*package));

        if *packages.first()? > side_weight {
            // Invalid configuration: the biggest item won't fit into any group
            return None;
        }

        Some(Configurator {
            packages: Rc::new(packages),
            side_weight,
            use_trunk,
        })
    }

    fn packing_list(&self, compartments: Vec<Compartment>) -> PackingList {
        PackingList {
            configurator: &self,
            compartments,
        }
    }

    /// Generate a triplet of packing lists.
    ///
    /// The first two lists share the same sum. The last is whatever is left.
    fn generate_groups_3(&self) -> impl '_ + Iterator<Item = Group3> {
        let package_set: HashSet<_> = self.packages.iter().copied().collect();
        let side_weight = self.side_weight;
        BoundedPermutationGenerator::new_rc(self.packages.clone(), self.side_weight)
            .flat_map(move |footwell| {
                let footwell = Rc::new(footwell);
                let footwell_set = footwell.iter().copied().collect();

                // compute the packages still available in the format required by BPG
                let diff_iter = package_set.difference(&footwell_set).copied();
                let mut still_available: Vec<_> = diff_iter.clone().collect();
                still_available.sort_unstable_by_key(|package| Reverse(*package));
                let still_available_set: HashSet<_> = diff_iter.collect();

                BoundedPermutationGenerator::new_rc(Rc::new(still_available), side_weight).map(
                    move |left_saddle| {
                        let left_saddle = Rc::new(left_saddle);
                        let left_saddle_set = left_saddle.iter().copied().collect();

                        let mut leftovers: Vec<_> = still_available_set
                            .difference(&left_saddle_set)
                            .copied()
                            .collect();
                        leftovers.sort_unstable_by_key(|package| Reverse(*package));

                        (footwell.clone(), left_saddle, Rc::new(leftovers))
                    },
                )
            })
            .filter(|(footwell, left_saddle, _)| {
                footwell.iter().sum::<Package>() == left_saddle.iter().sum::<Package>()
            })
    }

    /// Generate a quadruplet of packing lists.
    ///
    /// The first three lists share the same sum.
    fn generate_groups_4(&self) -> impl '_ + Iterator<Item = Group4> {
        self.generate_groups_3()
            .flat_map(move |(footwell, left_saddle, leftovers)| {
                let leftovers_set: HashSet<_> = leftovers.iter().copied().collect();

                BoundedPermutationGenerator::new_rc(leftovers, self.side_weight).map(
                    move |right_saddle| {
                        let footwell = footwell.clone();
                        let left_saddle = left_saddle.clone();
                        let right_saddle = Rc::new(right_saddle);
                        let right_saddle_set = right_saddle.iter().copied().collect();

                        let mut trunk: Vec<_> = leftovers_set
                            .difference(&right_saddle_set)
                            .copied()
                            .collect();
                        trunk.sort_unstable_by_key(|package| Reverse(*package));

                        (footwell, left_saddle, right_saddle, Rc::new(trunk))
                    },
                )
            })
            .filter(|(_, left_saddle, right_saddle, _)| {
                left_saddle.iter().sum::<Package>() == right_saddle.iter().sum::<Package>()
            })
    }

    fn generate_groups(&self) -> Box<dyn '_ + Iterator<Item = Group4>> {
        if self.use_trunk {
            Box::new(self.generate_groups_4())
        } else {
            Box::new(
                self.generate_groups_3()
                    .map(|(footwell, left_saddle, right_saddle)| {
                        (footwell, left_saddle, right_saddle, Rc::new(Vec::new()))
                    }),
            )
        }
    }

    /// Generate a sequence of packing lists satisfying the given balance constraints.
    pub fn packing_lists(&self) -> impl '_ + Iterator<Item = PackingList> {
        self.generate_groups()
            .map(move |(footwell, left_saddle, right_saddle, trunk)| {
                self.packing_list(compartments_from_groups(
                    footwell.as_slice(),
                    left_saddle.as_slice(),
                    right_saddle.as_slice(),
                    trunk.as_slice(),
                ))
            })
    }

    /// Determine the best sleigh configuration for the given packages.
    ///
    /// The best sleigh configuration is the one for which `sleigh.foot.len()` is minimal. If
    /// multiple sleighs can be configured with equal numbers of items in the footwells, the best
    /// of those is the one for which `sleigh.foot_qe()` is minimal.
    pub fn best(&self) -> Option<PackingList> {
        self.packing_lists()
            .map(|packing_list| {
                (
                    packing_list.packages_in(Compartment::Footwell).count(),
                    packing_list.qe(Compartment::Footwell),
                    packing_list,
                )
            })
            .min()
            .map(|(_, _, packing_list)| packing_list)
    }
}
