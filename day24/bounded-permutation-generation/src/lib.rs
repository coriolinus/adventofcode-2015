use std::{cmp::Ordering, ops::Sub};

pub trait Permutable: Copy + Ord + Sub<Output = Self> {}
impl<T: Copy + Ord + Sub<Output = Self>> Permutable for T {}

/// A `BoundedPermutationGenerator` efficiently generates selections of packages having the required sum.
///
/// # Method of operation
///
/// The `packages` slice is a reverse-sorted list of available packages. `package_idx` is an index
/// into that slice.
///
/// `queue` is a mutable reference to a scratchpad vector, which can be passed to recursive elements
/// as necessary.
///
/// At each level of recursion, there is a loop considering each index in turn. For each iteration
/// of that loop, the generator recursively attempts to produce a set summing to the desired target.
///
/// The recursion provides efficient backtracking.
///
/// Recursion termination conditions:
///
/// - if `idx >= self.packages.len()`, we have not achieved a sufficient sum; unwind
/// - if we have discovered a set of packages with the desired size, increment `package_idx`, clone the scratchpad and return
//
// Note the interior mutability here. It's standing in for what, in a more generator-friendly world,
// would be mutable local stack variables. However, we don't really have much better option here than
// to overrule the mutability portion of the borrow checker.
#[derive(Debug)]
pub struct BoundedPermutationGenerator<'a, T, Compartment> {
    packages: &'a [T],
    compartment_layout: &'a mut [Option<Compartment>],
    package_idx: usize,
    target_sum: T,
    child: Option<Box<BoundedPermutationGenerator<'a, T, Compartment>>>,
}

impl<'a, T, Compartment> BoundedPermutationGenerator<'a, T, Compartment>
where
    T: Permutable,
    Compartment: Copy + Eq,
{
    /// Create a new `BoundedPermutationGenerator` from a list of packages.
    ///
    /// # Preconditions
    ///
    /// - `packges` must be reverse-sorted.
    /// - `compartment_layout` must be at least as long as `packages`.
    pub fn new(
        packages: &'a [T],
        compartment_layout: &'a mut [Option<Compartment>],
        target_sum: T,
    ) -> Result<BoundedPermutationGenerator<'a, T, Compartment>, Error> {
        if !packages.windows(2).all(|window| window[1] <= window[0]) {
            return Err(Error::PackagesNotSorted);
        }
        if compartment_layout.len() < packages.len() {
            return Err(Error::CompartmentLayoutTooSmall);
        }
        Ok(BoundedPermutationGenerator {
            packages,
            compartment_layout,
            target_sum,
            package_idx: 0,
            child: None,
        })
    }

    /// Create a child generator which can be used to recursively seek solutions.
    fn child(&mut self) -> Box<BoundedPermutationGenerator<T, Compartment>> {
        Box::new(BoundedPermutationGenerator {
            packages: self.packages,
            compartment_layout: self.compartment_layout,
            target_sum: self.target_sum - self.packages[self.package_idx],
            package_idx: self.package_idx + 1,
            child: None,
        })
    }

    /// Recursively generate the next valid layout for members of this compartment.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.compartment_layout`.
    fn next_solution_for(
        &'a mut self,
        compartment: Compartment,
    ) -> Option<Vec<Option<Compartment>>> {
        let mut solution = None;
        while solution.is_none() {
            match &mut self.child {
                None => {
                    // no child generator means that we should compare indices at this level.
                    if self.package_idx >= self.packages.len() {
                        // we've exhausted the packages available
                        break;
                    }

                    if let Some(existing_compartment) = self.compartment_layout[self.package_idx] {
                        if existing_compartment == compartment {
                            // we've re-entered after returning a valid solution.
                            // To avoid infinite loops, unset this value and try the next.
                            self.compartment_layout[self.package_idx] = None;
                        }
                        // otherwise never overwrite a previously-set member of the compartment layout.
                        // this property is essential for composability.
                        self.package_idx += 1;
                        continue;
                    }

                    match self.packages[self.package_idx].cmp(&self.target_sum) {
                        Ordering::Greater => {
                            // no luck; try the next one
                            self.package_idx += 1;
                        }
                        Ordering::Equal => {
                            // we've identified a legal package set. We're going
                            // to return it, but preserving all struct state so
                            // that we can resume from this point without issue.
                            self.compartment_layout[self.package_idx] = Some(compartment);
                            solution = Some(self.compartment_layout.to_vec());
                        }
                        Ordering::Less => {
                            // recursively try different subsets
                            self.compartment_layout[self.package_idx] = Some(compartment);
                            self.child = Some(self.child());
                        }
                    }
                }
                Some(ref mut child) => {
                    match child.next_solution_for(compartment) {
                        // while the child produces solutions, just pass them along.
                        Some(inner_solution) => {
                            solution = Some(inner_solution);
                        }
                        // Note that most cleanup is actually handled by the reentrant cleanup above.
                        None => self.child = None,
                    }
                }
            }
        }
        solution
    }
}

// impl<T> Iterator for BoundedPermutationGenerator<T>
// where
//     T: Permutable,
// {
//     type Item = Vec<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next_solution_for()
//     }
// }

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
enum Error {
    #[error("`packages` input was not sorted")]
    PackagesNotSorted,
    #[error("`compartment_layout` was too small")]
    CompartmentLayoutTooSmall,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutations_8() {
        let values = Rc::new(vec![5, 3, 2, 1]);
        assert_eq!(
            BoundedPermutationGenerator::new_rc(values, 8).collect::<Vec<_>>(),
            vec![vec![5, 3], vec![5, 2, 1]],
        );
    }

    #[test]
    fn test_permutations_6() {
        let values = Rc::new(vec![5, 3, 2, 1]);
        assert_eq!(
            BoundedPermutationGenerator::new_rc(values, 6).collect::<Vec<_>>(),
            vec![vec![5, 1], vec![3, 2, 1]],
        );
    }
}
