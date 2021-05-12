use std::{cmp::Ordering, marker::PhantomData, ops::Sub};

pub trait Permutable: Copy + Ord + Sub<Output = Self> {}
impl<T: Copy + Ord + Sub<Output = Self>> Permutable for T {}

type Solution<Compartment> = Vec<Option<Compartment>>;

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
    _lifetime: PhantomData<&'a ()>,
    inner: Inner<T, Compartment>,
}

/// The inner structure contains all the actual implementation details of the solution generator.
///
/// It's a separate, private struct because it uses raw pointers instead of normal references.
/// This is because it's recursive, and rustc can't figure out an appropriate lifetime otherwise.
///
/// Imagine that we hadn't separated the lifetime from the references: it would look like
///
/// ```
/// pub struct BoundedPermutationGenerator<'a, T, Compartment> {
///   packages: &'a [T],
///   compartment_layout: &'a mut [Option<Compartment>],
///   package_idx: usize,
///   target_sum: T,
///   child: Option<Box<BoundedPermutationGenerator<'a, T, Compartment>>>,
/// }
/// ```
///
/// The problem is the `child` field: because we've defined it to be `'a`, then any borrow _must_
/// last for that long, which doesn't work with the recursive strategy that we want to use. However,
/// we don't have access to any other lifetime which we can use.
///
/// Splitting the lifetime away means that we have to do a little more work to ensure that everything
/// stays safe, but it also means that this minimal-copy approach is possible at all.
#[derive(Debug)]
struct Inner<T, Compartment> {
    packages: *const [T],
    compartment_layout: *mut [Option<Compartment>],
    package_idx: usize,
    target_sum: T,
    child: Option<Box<Inner<T, Compartment>>>,
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
            _lifetime: PhantomData,
            inner: Inner {
                packages: packages as _,
                compartment_layout: compartment_layout as _,
                target_sum,
                package_idx: 0,
                child: None,
            },
        })
    }

    /// Recursively generate the next valid layout for members of this compartment.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.compartment_layout`.
    pub fn next_solution_for(&mut self, compartment: Compartment) -> Option<Solution<Compartment>> {
        self.inner.next_solution_for(compartment)
    }

    /// Iterate over the remaining solutions of this generator.
    pub fn iter<'b>(&'b mut self, compartment: Compartment) -> Iter<'a, 'b, T, Compartment> {
        Iter {
            bpg: self,
            compartment,
        }
    }
}

impl<T, Compartment> Inner<T, Compartment>
where
    T: Permutable,
    Compartment: Copy + Eq,
{
    /// Private access to `self.packages` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn packages(&self) -> &[T] {
        unsafe { &*self.packages }
    }

    /// Private access to `self.compartment_layout` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn compartment_layout(&self) -> &[Option<Compartment>] {
        unsafe { &*self.compartment_layout }
    }

    /// Private mutable access to `self.compartment_layout` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn compartment_layout_mut(&self) -> &mut [Option<Compartment>] {
        unsafe { &mut *self.compartment_layout }
    }

    /// Create a child generator which can be used to recursively seek solutions.
    fn child(&mut self) -> Box<Self> {
        Box::new(Self {
            packages: self.packages,
            compartment_layout: self.compartment_layout,
            target_sum: self.target_sum - self.packages()[self.package_idx],
            package_idx: self.package_idx + 1,
            child: None,
        })
    }

    /// Recursively generate the next valid layout for members of this compartment.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.compartment_layout`.
    fn next_solution_for(&mut self, compartment: Compartment) -> Option<Vec<Option<Compartment>>> {
        let mut solution = None;
        while solution.is_none() {
            self.child = match self.child.take() {
                None => {
                    // no child generator means that we should compare indices at this level.
                    if self.package_idx >= self.packages().len() {
                        // we've exhausted the packages available
                        break;
                    }

                    if let Some(existing_compartment) = self.compartment_layout()[self.package_idx]
                    {
                        if existing_compartment == compartment {
                            // we've re-entered after returning a valid solution.
                            // To avoid infinite loops, unset this value and try the next.
                            self.compartment_layout_mut()[self.package_idx] = None;
                        }
                        // otherwise never overwrite a previously-set member of the compartment layout.
                        // this property is essential for composability.
                        self.package_idx += 1;
                        continue;
                    }

                    match self.packages()[self.package_idx].cmp(&self.target_sum) {
                        Ordering::Greater => {
                            // no luck; try the next one
                            self.package_idx += 1;
                            None
                        }
                        Ordering::Equal => {
                            // we've identified a legal package set. We're going
                            // to return it, but preserving all struct state so
                            // that we can resume from this point without issue.
                            self.compartment_layout_mut()[self.package_idx] = Some(compartment);
                            solution = Some(self.compartment_layout().to_vec());
                            None
                        }
                        Ordering::Less => {
                            // recursively try different subsets
                            self.compartment_layout_mut()[self.package_idx] = Some(compartment);
                            Some(self.child())
                        }
                    }
                }
                Some(mut child) => {
                    // can't use `map` here because the borrow checker gets upset about the lifetime
                    // as `child` moves through the closure.
                    match child.next_solution_for(compartment) {
                        Some(inner_solution) => {
                            // while the child produces solutions, just pass them along.
                            solution = Some(inner_solution);
                            Some(child)
                        }
                        // If next_solution_for produces None, then `next_child` becomes None, engaging
                        // cleanup once the loop cycles through to the next iteration.
                        None => None,
                    }
                }
            };
        }
        solution
    }
}

pub struct Iter<'a, 'b, T, Compartment>
where
    'a: 'b,
{
    bpg: &'b mut BoundedPermutationGenerator<'a, T, Compartment>,
    compartment: Compartment,
}

impl<'a, 'b, T, Compartment> Iterator for Iter<'a, 'b, T, Compartment>
where
    'a: 'b,
    T: Permutable,
    Compartment: Copy + Eq,
{
    type Item = Solution<Compartment>;

    fn next(&mut self) -> Option<Self::Item> {
        self.bpg.next_solution_for(self.compartment)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
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
        let values = vec![5, 3, 2, 1];
        let mut compartment_layout = vec![None; values.len()];
        let mut bpg =
            BoundedPermutationGenerator::new(&values, &mut compartment_layout, 8).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), Some(0), None, None]);

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, Some(0), Some(0)]);

        assert!(bpg.next_solution_for(0).is_none());
    }

    #[test]
    fn test_permutations_6() {
        let values = vec![5, 3, 2, 1];
        let mut compartment_layout = vec![None; values.len()];
        let mut bpg =
            BoundedPermutationGenerator::new(&values, &mut compartment_layout, 6).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, None, Some(0)]);

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![None, Some(0), Some(0), Some(0)]);

        assert!(bpg.next_solution_for(0).is_none());
    }

    #[test]
    fn test_safe_existing_data() {
        let values = vec![5, 3, 2, 1];
        let mut compartment_layout = vec![None; values.len()];
        let mut bpg =
            BoundedPermutationGenerator::new(&values, &mut compartment_layout, 5).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, None, None]);
        drop(bpg);

        assert_eq!(compartment_layout, solution);

        let mut bpg =
            BoundedPermutationGenerator::new(&values, &mut compartment_layout, 5).unwrap();

        let solution = bpg.next_solution_for(1).unwrap();
        assert_eq!(solution, vec![Some(0), Some(1), Some(1), None]);

        assert!(bpg.next_solution_for(1).is_none());
    }

    #[test]
    fn test_iteration_6() {
        let values = vec![5, 3, 2, 1];
        let mut compartment_layout = vec![None; values.len()];
        let mut bpg =
            BoundedPermutationGenerator::new(&values, &mut compartment_layout, 6).unwrap();

        let expect_solutions = vec![
            vec![Some(0), None, None, Some(0)],
            vec![None, Some(0), Some(0), Some(0)],
        ];

        let solutions = bpg.iter(0).collect::<Vec<_>>();
        assert_eq!(solutions, expect_solutions);
    }
}
