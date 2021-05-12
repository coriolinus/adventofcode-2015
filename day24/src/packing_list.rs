use crate::{Compartment, Configurator, Package};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackingList<'a> {
    pub(crate) configurator: &'a Configurator<'a>,
    pub(crate) compartments: Vec<Option<Compartment>>,
}

impl<'a> PackingList<'a> {
    pub fn packages_in(&self, want: Compartment) -> impl '_ + Iterator<Item = Package> {
        self.configurator
            .packages
            .iter()
            .zip(self.compartments.iter())
            .filter_map(move |(package, assignment)| {
                assignment.and_then(|have| (have == want).then(|| *package))
            })
    }

    /// Quantum Entanglement of the footwell of this sleigh.
    pub fn qe(&self, compartment: Compartment) -> u64 {
        self.packages_in(compartment)
            .map(|weight| weight as u64)
            .product()
    }
}
