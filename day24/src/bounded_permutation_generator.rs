use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{DerefMut, Sub},
    pin::Pin,
};

pub trait Permutable: Copy + Ord + Sub<Output = Self> {}
impl<T: Copy + Ord + Sub<Output = Self>> Permutable for T {}

type Solution<Subset> = Vec<Option<Subset>>;

/// A `BoundedPermutationGenerator` efficiently generates distinct subsets of an input list of items,
/// where each subset sums to a particular value.
///
/// Its output is `Solution`: a `Vec<Option<Subset>>`. A non-`None` entry in the `Solution`
/// indicates that the corresponding value in `items` should be assigned to that subset.
///
/// It can accept a partial solution with [`from_solution`][Self::from_solution]. In that case,
/// [`next_solution_for`][Self::next_solution_for] will never select a value already used for a
/// different subset.
///
/// Note that the constructors always produce `Pin<Box<Self>>`. This is an implementation detail.
/// To function efficiently, this contains (many) references to an area of memory used as a scratch
/// space. If it were possible to use an external slice for this, then we wouldn't need to pin.
/// However, we strongly desire that it's possible to construct a free-standing generator from this;
/// that implies that this struct needs to own its working memory. That means that those
/// references must be internal, which in turn means that we need to ensure that this struct never
/// moves so that the internal references remain valid.
#[derive(Debug, Clone)]
pub struct BoundedPermutationGenerator<'a, T, Subset> {
    _lifetime: PhantomData<&'a ()>,
    scratch_space: Solution<Subset>,
    inner: Inner<T, Subset>,
}

impl<'a, T, Subset> BoundedPermutationGenerator<'a, T, Subset>
where
    T: Permutable,
    Subset: Copy + Eq,
{
    /// Create a new `BoundedPermutationGenerator` from a list of items.
    ///
    /// # Preconditions
    ///
    /// - `packges` must be reverse-sorted.
    pub fn new(items: &'a [T], target_sum: T) -> Result<Pin<Box<Self>>, Error> {
        Self::from_solution(items, target_sum, vec![None; items.len()])
    }

    /// Create a new `BoundedPermutationGenerator` from a list of items and an existing partial solution.
    ///
    /// # Preconditions
    ///
    /// - `items` must be reverse-sorted.
    /// - `solution.len()` must equal `items.len()`.
    pub fn from_solution(
        items: &'a [T],
        target_sum: T,
        solution: Solution<Subset>,
    ) -> Result<Pin<Box<Self>>, Error> {
        if solution.len() != items.len() {
            return Err(Error::WrongSolutionSize(solution.len(), items.len()));
        }
        if !items.windows(2).all(|window| window[1] <= window[0]) {
            return Err(Error::ItemsNotSorted);
        }
        let mut temporary = Vec::new();
        // we'll update the `subset_layout` field to be non-null once the outer is pinned
        let bpg = BoundedPermutationGenerator {
            _lifetime: PhantomData,
            scratch_space: solution,
            inner: Inner {
                items: items as _,
                scratch_space: temporary.as_mut_slice() as _,
                target_sum,
                idx: 0,
                child: None,
            },
        };
        let mut boxed = Box::pin(bpg);
        let scratch_space = boxed.scratch_space.as_slice() as *const _ as _;

        // safe because modifying a field never moves the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).inner.scratch_space = scratch_space;
        }

        Ok(boxed)
    }

    /// Generate the next valid layout for members of this subset.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.subset_layout`.
    pub fn next_solution_for(&mut self, subset: Subset) -> Option<Solution<Subset>> {
        self.inner.next_solution_for(subset)
    }

    /// Transform into an iterator over the remaining solutions of this generator.
    pub fn into_iter(self: Pin<Box<Self>>, subset: Subset) -> Iter<'a, T, Subset> {
        Iter { bpg: self, subset }
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
/// pub struct BoundedPermutationGenerator<'a, T, Subset> {
///   items: &'a [T],
///   scratch_space: &'a mut [Option<Subset>],
///   item_idx: usize,
///   target_sum: T,
///   child: Option<Box<BoundedPermutationGenerator<'a, T, Subset>>>,
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
struct Inner<T, Subset> {
    /// a reverse-sorted list of available items.
    items: *const [T],
    /// working space containing the current state of the partial solution.
    scratch_space: *mut [Option<Subset>],
    /// an index into `self.items` and `self.subset_layout`.
    idx: usize,
    /// keeps track of the value we're looking for at this depth of recursion.
    target_sum: T,
    /// if present, a recursive child assuming that the current item is part of the solution.
    child: Option<Box<Inner<T, Subset>>>,
}

impl<T, Subset> Inner<T, Subset>
where
    T: Permutable,
    Subset: Copy + Eq,
{
    /// Private access to `self.items` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn items(&self) -> &[T] {
        unsafe { &*self.items }
    }

    /// Private access to `self.subset_layout` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn subset_layout(&self) -> &[Option<Subset>] {
        unsafe { &*self.scratch_space }
    }

    /// Private mutable access to `self.subset_layout` as a slice.
    ///
    /// Safe because the only way to construct a `BoundedPermutationGenerator` requires a valid slice,
    /// and we never edit the pointer.
    fn subset_layout_mut(&self) -> &mut [Option<Subset>] {
        unsafe { &mut *self.scratch_space }
    }

    /// Create a child generator which can be used to recursively seek solutions.
    fn child(&mut self) -> Box<Self> {
        Box::new(Self {
            items: self.items,
            scratch_space: self.scratch_space,
            target_sum: self.target_sum - self.items()[self.idx],
            idx: self.idx + 1,
            child: None,
        })
    }

    /// Recursively generate the next valid layout for members of this subset.
    ///
    /// Each solution requires an allocation and data-copying proportional to `self.subset_layout`.
    ///
    /// # Method of operation
    ///
    /// - if we have a child, produce its next solution
    /// - if we don't have a child, for each index available to us:
    ///   - if it's greater than the target, then just try the next one
    ///   - if it's equal to the target, then we know the solution is complete
    ///   - if it's less than the target, create a child which will attempt to generate solutions
    ///     assuming that the current item is a member of the solution set
    ///
    /// Because this is recursive and cleans up after itself, the stack provides efficient backtracking.
    fn next_solution_for(&mut self, subset: Subset) -> Option<Solution<Subset>> {
        let mut solution = None;
        while solution.is_none() && self.idx < self.items().len() {
            match self.child {
                None => {
                    if let Some(existing_subset) = self.subset_layout()[self.idx] {
                        if existing_subset == subset {
                            // we've re-entered after returning a valid solution.
                            // To avoid infinite loops, unset this value and try the next.
                            self.subset_layout_mut()[self.idx] = None;
                        }
                        // otherwise never overwrite a previously-set member of the subset layout.
                        // this property is essential for composability.
                        self.idx += 1;
                        continue;
                    }

                    match self.items()[self.idx].cmp(&self.target_sum) {
                        Ordering::Greater => {
                            // no luck; try the next one
                            self.idx += 1;
                        }
                        Ordering::Equal => {
                            // we've identified a valid solution.
                            self.subset_layout_mut()[self.idx] = Some(subset);
                            solution = Some(self.subset_layout().to_vec());
                        }
                        Ordering::Less => {
                            // recursively try different subsets assuming this item is a member of
                            // the solution.
                            self.subset_layout_mut()[self.idx] = Some(subset);
                            self.child = Some(self.child());
                        }
                    }
                }
                Some(ref mut child) => {
                    match child.next_solution_for(subset) {
                        Some(inner_solution) => {
                            // while the child produces solutions, just pass them along.
                            solution = Some(inner_solution);
                        }
                        // If the child stops producing values, unset it; the next iteration of the
                        // main loop will clean up the rest.
                        None => self.child = None,
                    }
                }
            };
        }
        solution
    }
}

pub struct Iter<'a, T, Subset> {
    bpg: Pin<Box<BoundedPermutationGenerator<'a, T, Subset>>>,
    subset: Subset,
}

impl<'a, T, Subset> Iterator for Iter<'a, T, Subset>
where
    T: Permutable,
    Subset: Copy + Eq,
    Pin<Box<BoundedPermutationGenerator<'a, T, Subset>>>:
        DerefMut<Target = BoundedPermutationGenerator<'a, T, Subset>>,
{
    type Item = Solution<Subset>;

    fn next(&mut self) -> Option<Self::Item> {
        self.bpg.next_solution_for(self.subset)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("`items` input was not sorted")]
    ItemsNotSorted,
    #[error("Solution has {0} entries but must have {1} to match items")]
    WrongSolutionSize(usize, usize),
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
