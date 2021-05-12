use std::cmp::Reverse;

use crate::{
    bounded_permutation_generator::BoundedPermutationGenerator, Compartment, Package, PackingList,
};

type Solution = Vec<Option<Compartment>>;

/// Generator of legal sleigh configurations. Main entry point to this module.
///
/// Note: This only handles the case that all of the `Package`s have unique sizes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Configurator<'a> {
    // always reverse-sorted
    pub(crate) packages: &'a [Package],
    pub(crate) side_weight: Package, // weight for each side
    pub(crate) use_trunk: bool,
}

impl<'a> Configurator<'a> {
    /// Construct a new `SleighConfigurations` generator.
    ///
    /// Returns `None` if the total weight can't be evenly divided by 3, or if the biggest package
    /// is bigger than 1/3 of the total weight, because in those circumstances no valid sleigh
    /// configurations can be generated.
    pub fn new(packages: &mut [Package], use_trunk: bool) -> Option<Configurator> {
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
            packages,
            side_weight,
            use_trunk,
        })
    }

    fn packing_list(&self, compartments: Solution) -> PackingList {
        PackingList {
            configurator: &self,
            compartments,
        }
    }

    /// Generate partial solutions for which the trunk is not considered.
    ///
    /// In these solutions the footwell is assigned. The sides have not been fully assigned, but
    /// it has been demonstrated that at least one assignment is possible for the sides.
    ///
    /// The trunk is not used.
    fn generate_footwell_no_trunk(&self) -> impl '_ + Iterator<Item = Solution> {
        BoundedPermutationGenerator::new(&self.packages, self.side_weight)
            .expect("sort guaranteed by the constructor")
            .into_iter(Compartment::Footwell)
            .filter_map(move |partial_solution| {
                // we need to ensure that it is possible to generate at least one full solution from
                // this partial solution, but we don't need to bother actually generating more than
                // one.
                BoundedPermutationGenerator::from_solution(
                    &self.packages,
                    self.side_weight,
                    partial_solution,
                )
                .expect("sort guaranteed by the constructor")
                .into_iter(Compartment::LeftSaddle)
                .next()
            })
    }

    /// Generate partial solutions for which the trunk is considered.
    ///
    /// In these solutions the footwell is assigned. The sides have not been fully assigned, but
    /// it has been demonstrated that at least one assignment is possible for the sides.
    ///
    /// The trunk is used.
    fn generate_footwell_with_trunk(&self) -> impl '_ + Iterator<Item = Solution> {
        self.generate_footwell_no_trunk()
            .filter_map(move |partial_solution| {
                // we need to ensure that it is possible to generate at least one full solution from
                // this partial solution, but we don't need to bother actually generating more than
                // one.
                BoundedPermutationGenerator::from_solution(
                    &self.packages,
                    self.side_weight,
                    partial_solution,
                )
                .expect("sort guaranteed by the constructor")
                .into_iter(Compartment::RightSaddle)
                .next()
            })
    }

    /// Generate complete solutions, with or without the trunk as appropriate.
    ///
    /// Note that not all complete solutions are generated. The footwell's solutions are exhaustively
    /// generated, but all other compartments only have demonstration solutions.
    fn generate_footwell(&self) -> Box<dyn '_ + Iterator<Item = Solution>> {
        if self.use_trunk {
            Box::new(self.generate_footwell_with_trunk().map(|mut partial| {
                partial.iter_mut().for_each(|item| {
                    if item.is_none() {
                        *item = Some(Compartment::Trunk)
                    }
                });
                partial
            }))
        } else {
            Box::new(self.generate_footwell_no_trunk().map(|mut partial| {
                partial.iter_mut().for_each(|item| {
                    if item.is_none() {
                        *item = Some(Compartment::RightSaddle)
                    }
                });
                partial
            }))
        }
    }

    /// Generate a sequence of packing lists satisfying the given balance constraints.
    pub fn packing_lists(&self) -> impl '_ + Iterator<Item = PackingList> {
        self.generate_footwell()
            .map(move |solution| self.packing_list(solution))
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
