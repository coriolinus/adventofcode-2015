use crate::Package;
use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    rc::Rc,
};

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
pub struct BoundedPermutationGenerator {
    packages: Rc<Vec<Package>>,
    queue: Rc<RefCell<Vec<Package>>>,
    package_idx: Cell<usize>,
    target_sum: Package,
    child: RefCell<Option<Box<BoundedPermutationGenerator>>>,
}

impl BoundedPermutationGenerator {
    /// Create a new `BoundedPermutationGenerator` from a cheaply-cloneable sorted vec of packages.
    ///
    /// # Panic
    ///
    /// Panics if `packages` is not reverse-sorted.
    pub fn new_rc(packages: Rc<Vec<Package>>, target_sum: Package) -> BoundedPermutationGenerator {
        debug_assert!(packages.windows(2).all(|window| window[1] <= window[0]));
        BoundedPermutationGenerator {
            packages,
            queue: Rc::new(RefCell::new(Vec::new())),
            package_idx: Cell::new(0),
            target_sum,
            child: RefCell::new(None),
        }
    }

    /// Create a child generator which can be used to recursively seek solutions.
    fn child(&self) -> Box<BoundedPermutationGenerator> {
        let package_idx = Cell::new(self.package_idx.get() + 1);
        let target_sum = self.target_sum - self.packages[self.package_idx.get()];

        Box::new(BoundedPermutationGenerator {
            packages: self.packages.clone(),
            queue: self.queue.clone(),
            target_sum,
            package_idx,
            child: RefCell::new(None),
        })
    }

    fn incr_idx(&self) {
        self.package_idx.set(self.package_idx.get() + 1);
    }

    /// Recursively seek the next valid package set.
    fn next_solution(&self) -> Option<Vec<Package>> {
        let mut solution = None;
        while solution.is_none() {
            match *self.child.borrow() {
                None => {
                    // no child generator means that we should compare indices at this level.
                    if self.package_idx.get() >= self.packages.len() {
                        // we've exhausted the packages available
                        break;
                    }

                    let package = self.packages[self.package_idx.get()];

                    match package.cmp(&self.target_sum) {
                        Ordering::Greater => {
                            // as the packages list is reverse-sorted, we know
                            // we can't possibly produce any more legal package
                            // sets from this iteration.
                            break;
                        }
                        Ordering::Equal => {
                            // we've identified a legal package set. We're going
                            // to return it, but preserving all struct state so
                            // that we can resume from this point without issue.
                            solution = Some({
                                let mut solution = self.queue.borrow().clone();
                                solution.push(package);
                                solution
                            });
                            self.incr_idx();
                        }
                        Ordering::Less => {
                            // recursively try different subsets
                            self.queue.borrow_mut().push(package);
                            self.child.replace(Some(self.child()));
                            // note that we don't pop in this section of code. Instead,
                            // we loop back through the `Some(child)` branch until the child
                            // is exhausted, and then pop there.
                        }
                    }
                }
                Some(ref child) => {
                    match child.next_solution() {
                        // while the child produces solutions, just pass them along.
                        Some(inner_solution) => solution = Some(inner_solution),
                        // afterwards, clean up.
                        None => {
                            std::mem::drop(child);
                            self.child.replace(None);
                            self.queue.borrow_mut().pop();
                            self.incr_idx();
                        }
                    }
                }
            }
        }
        solution
    }
}

impl Iterator for BoundedPermutationGenerator {
    type Item = Vec<Package>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_solution()
    }
}
