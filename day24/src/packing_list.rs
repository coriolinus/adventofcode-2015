use std::convert::TryInto;

use crate::{Compartment, Configurator, Package};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackingList<'a> {
    pub(crate) configurator: &'a Configurator,
    pub(crate) compartments: Vec<Compartment>,
}

impl<'a> PackingList<'a> {
    pub fn packages_in(&self, want: Compartment) -> impl '_ + Iterator<Item = Package> {
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

pub(crate) fn compartments_from_groups(
    footwell: &[Package],
    left_saddle: &[Package],
    right_saddle: &[Package],
    trunk: &[Package],
) -> Vec<Compartment> {
    let mut compartments =
        Vec::with_capacity(footwell.len() + left_saddle.len() + right_saddle.len() + trunk.len());

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
