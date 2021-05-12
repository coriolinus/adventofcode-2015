use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{DerefMut, Sub},
    pin::Pin,
};

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
/// - if we have discovered a set of packages with the desired size, increment `package_idx`, clone
///   the scratchpad and return
//
// We have to get fancy with pinning because this generator is necessarily self-referential:
// `inner` contains a reference to `scratch_space`.
#[derive(Debug, Clone)]
pub struct BoundedPermutationGenerator<'a, T, Compartment> {
    _lifetime: PhantomData<&'a ()>,
    scratch_space: Solution<Compartment>,
    inner: Inner<T, Compartment>,
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
    pub fn new(
        packages: &'a [T],
        target_sum: T,
    ) -> Result<Pin<Box<BoundedPermutationGenerator<'a, T, Compartment>>>, Error> {
        Self::from_solution(packages, target_sum, vec![None; packages.len()])
    }

    /// Create a new `BoundedPermutationGenerator` from a list of packages and an existing solution.
    ///
    /// # Preconditions
    ///
    /// - `packages` must be reverse-sorted.
    pub fn from_solution(
        packages: &'a [T],
        target_sum: T,
        solution: Solution<Compartment>,
    ) -> Result<Pin<Box<BoundedPermutationGenerator<'a, T, Compartment>>>, Error> {
        if !packages.windows(2).all(|window| window[1] <= window[0]) {
            return Err(Error::PackagesNotSorted);
        }
        let mut temporary = Vec::new();
        // we'll update the `compartment_layout` field to be non-null once the outer is pinned
        let bpg = BoundedPermutationGenerator {
            _lifetime: PhantomData,
            scratch_space: solution,
            inner: Inner {
                packages: packages as _,
                compartment_layout: temporary.as_mut_slice() as _,
                target_sum,
                package_idx: 0,
                child: None,
            },
        };
        let mut boxed = Box::pin(bpg);
        let scratch_space = boxed.scratch_space.as_slice() as *const _ as _;

        // safe because modifying a field never moves the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).inner.compartment_layout = scratch_space;
        }

        Ok(boxed)
    }

    /// Recursively generate the next valid layout for members of this compartment.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.compartment_layout`.
    pub fn next_solution_for(&mut self, compartment: Compartment) -> Option<Solution<Compartment>> {
        self.inner.next_solution_for(compartment)
    }

    /// Transform into an iterator over the remaining solutions of this generator.
    pub fn into_iter(self: Pin<Box<Self>>, compartment: Compartment) -> Iter<'a, T, Compartment> {
        Iter {
            bpg: self,
            compartment,
        }
    }
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
#[derive(Debug, Clone)]
struct Inner<T, Compartment> {
    packages: *const [T],
    compartment_layout: *mut [Option<Compartment>],
    package_idx: usize,
    target_sum: T,
    child: Option<Box<Inner<T, Compartment>>>,
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
            match self.child {
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
                        }
                        Ordering::Equal => {
                            // we've identified a legal package set. We're going
                            // to return it, but preserving all struct state so
                            // that we can resume from this point without issue.
                            self.compartment_layout_mut()[self.package_idx] = Some(compartment);
                            solution = Some(self.compartment_layout().to_vec());
                        }
                        Ordering::Less => {
                            // recursively try different subsets
                            self.compartment_layout_mut()[self.package_idx] = Some(compartment);
                            self.child = Some(self.child());
                        }
                    }
                }
                Some(ref mut child) => {
                    // can't use `map` here because the borrow checker gets upset about the lifetime
                    // as `child` moves through the closure.
                    match child.next_solution_for(compartment) {
                        Some(inner_solution) => {
                            // while the child produces solutions, just pass them along.
                            solution = Some(inner_solution);
                        }
                        // If next_solution_for produces None, then `next_child` becomes None, engaging
                        // cleanup once the loop cycles through to the next iteration.
                        None => self.child = None,
                    }
                }
            };
        }
        solution
    }
}

pub struct Iter<'a, T, Compartment> {
    bpg: Pin<Box<BoundedPermutationGenerator<'a, T, Compartment>>>,
    compartment: Compartment,
}

impl<'a, T, Compartment> Iterator for Iter<'a, T, Compartment>
where
    T: Permutable,
    Compartment: Copy + Eq,
    Pin<Box<BoundedPermutationGenerator<'a, T, Compartment>>>:
        DerefMut<Target = BoundedPermutationGenerator<'a, T, Compartment>>,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutations_8() {
        let values = vec![5, 3, 2, 1];
        let mut bpg = BoundedPermutationGenerator::new(&values, 8).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), Some(0), None, None]);

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, Some(0), Some(0)]);

        assert!(bpg.deref_mut().next_solution_for(0).is_none());
    }

    #[test]
    fn test_permutations_6() {
        let values = vec![5, 3, 2, 1];
        let mut bpg = BoundedPermutationGenerator::new(&values, 6).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, None, Some(0)]);

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![None, Some(0), Some(0), Some(0)]);

        assert!(bpg.next_solution_for(0).is_none());
    }

    #[test]
    fn test_safe_existing_data() {
        let values = vec![5, 3, 2, 1];
        let mut bpg = BoundedPermutationGenerator::new(&values, 5).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, None, None]);

        let solution = bpg.next_solution_for(1).unwrap();
        assert_eq!(solution, vec![Some(0), Some(1), Some(1), None]);

        assert!(bpg.next_solution_for(1).is_none());
    }

    #[test]
    fn test_iteration_6() {
        let values = vec![5, 3, 2, 1];
        let bpg = BoundedPermutationGenerator::new(&values, 6).unwrap();

        let expect_solutions = vec![
            vec![Some(0), None, None, Some(0)],
            vec![None, Some(0), Some(0), Some(0)],
        ];

        let solutions = bpg.into_iter(0).collect::<Vec<_>>();
        assert_eq!(solutions, expect_solutions);
    }

    #[test]
    fn test_clone() {
        let values = vec![5, 3, 2, 1];
        let mut bpg = BoundedPermutationGenerator::new(&values, 5).unwrap();

        let solution = bpg.next_solution_for(0).unwrap();
        assert_eq!(solution, vec![Some(0), None, None, None]);

        let mut child = bpg.clone();

        let solution = child.next_solution_for(1).unwrap();
        assert_eq!(solution, vec![Some(0), Some(1), Some(1), None]);
        assert!(child.next_solution_for(1).is_none());

        let solution = bpg.next_solution_for(2).unwrap();
        assert_eq!(solution, vec![Some(0), Some(2), Some(2), None]);

        assert!(bpg.next_solution_for(2).is_none());
    }
}
