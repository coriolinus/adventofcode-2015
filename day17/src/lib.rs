//! # Day 17: No Such Thing as Too Much
//!
//! The elves bought too much eggnog again - `150` liters this time. To fit it all into your
//! refrigerator, you'll need to move it into smaller containers. You take an inventory of the
//! capacities of the available containers.
//!
//! For example, suppose you have containers of size `20`, `15`, `10`, `5`, and `5` liters. If you
//! need to store `25` liters, there are four ways to do it:
//!
//! - `15` and `10`
//! - `20` and `5` (the first `5`)
//! - `20` and `5` (the second `5`)
//! - `15`, `5`, and `5`
//!
//! Filling all containers entirely, how many different combinations of containers can exactly fit
//! all `150` liters of eggnog?

pub type Container = u8;

/// Recursively generates permutations of `Container`s whose capacities sum to a given volume.
#[derive(PartialEq, Eq, Clone)]
pub struct EggnogFiller {
    from: Container,
    into: Vec<Container>,
    biggest: Container,
    recursor: Option<Box<EggnogFiller>>,
    preserve_biggest: bool,
}

impl EggnogFiller {
    /// Construct a new EggnogFiller given an origin container and a *sorted* list of destination
    /// containers. These must be in order, largest to smallest, for this to work right.
    fn new_given_sorted(from: Container, into: Vec<Container>) -> EggnogFiller {
        EggnogFiller {
            from: from,
            into: into,
            biggest: 0,
            recursor: None,
            preserve_biggest: false,
        }
    }

    /// Construct a new EggnogFiller given an origin container and a list of destination containers.
    pub fn new(from: Container, into: Vec<Container>) -> EggnogFiller {
        let mut into = into;
        into.sort();
        into.reverse();
        EggnogFiller::new_given_sorted(from, into)
    }
}

impl Iterator for EggnogFiller {
    type Item = Vec<Container>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.into.len() == 0 {
            return None;
        }

        if !self.preserve_biggest {
            // let's modify the biggest number!
            self.biggest = self.into.remove(0);

            while self.biggest > self.from {
                if self.into.len() == 0 {
                    return None;
                }
                self.biggest = self.into.remove(0);
            }

            // Here, `self.biggest` is the largest element less than or equal to `self.from`, and all other
            // elements are guaranteed to be smaller than that.
            if self.biggest == self.from {
                // We really want this to be a yield, not a return.
                //
                // However, it should still work: For as long as there are items exactly as large
                // as the capacity we're trying to fill, we return them. Then, we proceed to the next
                // part, because `biggest` is now smaller than `from`.
                return Some(vec![self.biggest]);
            }
        }

        if self.into.len() > 0 {
            if self.recursor.is_none() {
                self.preserve_biggest = true;
                self.recursor = Some(Box::new(EggnogFiller::new_given_sorted(self.from -
                                                                             self.biggest,
                                                                             self.into
                                                                                 .clone())));
            }
            let mut clear_biggest = false;
            if let Some(ref mut sub_solution_iter) = self.recursor {
                match sub_solution_iter.next() {
                    None => {
                        // there are no more sub-solutions, therefore we have to move on.
                        // unfortunately, we can't do anything directly here due to ownership
                        // restrictions, but we can set a flag to take care of some necessary
                        // business once the borrows have ended.
                        clear_biggest = true;
                    }
                    Some(sub_solution) => {
                        // well, the presence of a sub-solution means that the items in this list
                        // add up to everything other than our biggest, so let's just add the
                        // biggest to the list and return it as a solution.
                        let mut ret = vec![self.biggest];
                        ret.extend(sub_solution);
                        return Some(ret);
                    }
                }
            }
            if clear_biggest {
                // This flag is set once the sub-solution iterator contained in `self.recursor`
                // is exhausted. The implication is that we can now move on to the next biggest
                // item in our list and start building its sub-solutions. Recursively calling
                // `self.next` is simply better than reiterating all that code down here.
                self.preserve_biggest = false;
                self.recursor = None;
                return self.next();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// For example, suppose you have containers of size `20`, `15`, `10`, `5`, and `5` liters. If you
    /// need to store `25` liters, there are four ways to do it:
    ///
    /// - `20` and `5` (the first `5`)
    /// - `20` and `5` (the second `5`)
    /// - `15` and `10`
    /// - `15`, `5`, and `5`
    #[test]
    fn test_example() {
        let mut filler = EggnogFiller::new(25, vec![20, 15, 10, 5, 5]);
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![15, 10]));
        assert_eq!(filler.next(), Some(vec![15, 5, 5]));
        assert_eq!(filler.next(), None);
    }

    #[test]
    fn test_example_reversed() {
        let mut filler = EggnogFiller::new(25, vec![5, 5, 10, 15, 20]);
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![15, 10]));
        assert_eq!(filler.next(), Some(vec![15, 5, 5]));
        assert_eq!(filler.next(), None);
    }

    #[test]
    fn test_example_munged() {
        let mut filler = EggnogFiller::new(25, vec![5, 10, 20, 15, 5]);
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![20, 5]));
        assert_eq!(filler.next(), Some(vec![15, 10]));
        assert_eq!(filler.next(), Some(vec![15, 5, 5]));
        assert_eq!(filler.next(), None);
    }
}
