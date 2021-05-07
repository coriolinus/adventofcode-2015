use std::fmt::Debug;
use std::ops;

/// An iterator over subsets of the numbers `N` such that the sum of each subset is equal to a given target.
///
/// Subsets will be returned in descending order of highest contained item.
/// The items within each returned subset will always be sorted in ascending order.
#[derive(Default)]
pub struct SummedSubsets<N> {
    items: Vec<N>,
    target: N,
    current_sum: N,
    sub_iterator: Option<Box<SummedSubsets<N>>>,
}

impl<N> SummedSubsets<N>
where
    N: Ord + Default + Debug,
{
    pub fn new(items: Vec<N>, target: N) -> SummedSubsets<N> {
        let mut items = items;
        items.sort();
        SummedSubsets {
            items: items,
            target: target,
            ..SummedSubsets::default()
        }
    }

    /// Get the next non-zero value <= the target
    fn next_value(&mut self) {
        // By default, our value is 0, which won't contribute to the weight. Pop to get items from
        // our list of items, until we get one which could possibly fit.
        //
        // Also, be sure not to just return the same thing but actually consume at least one value.
        self.current_sum = N::default();
        while self.current_sum == N::default() || self.current_sum > self.target {
            match self.items.pop() {
                Some(next_value) => self.current_sum = next_value,
                None => {
                    // we've run out of items
                    self.current_sum = N::default();
                    break;
                }
            }
        }
    }
}

impl<N> Iterator for SummedSubsets<N>
where
    N: Clone + Ord + Default + Debug + ops::Sub<Output = N>,
{
    type Item = Vec<N>;

    fn next(&mut self) -> Option<Vec<N>> {
        if let Some(ref mut sub_iterator) = self.sub_iterator {
            // I've got a sub-iterator going; let's continue it
            // Important implication: sub-iterators only exist when self.value is some
            // sane thing, not default()
            if let Some(sub_value) = sub_iterator.next() {
                // my sub-iterator returned a valid subsequence
                let mut ret = sub_value;
                ret.push(self.current_sum.clone());
                return Some(ret);
            }
        }

        // if we're here, either our sub-iterator has exhausted itself or we haven't constructed
        // one yet. Either way, we need to make one.
        self.sub_iterator = None;
        self.next_value();
        if self.current_sum == N::default() {
            // we're out of items
            return None;
        }
        // If we've found a perfect match, just return now instead of bothering with the sub-iterator.
        if self.current_sum == self.target {
            return Some(vec![self.current_sum.clone()]);
        }

        // Ok, self.value is greater than any item in self.items, but less than self.target. This
        // is when we need a sub-iterator.
        self.sub_iterator = Some(Box::new(SummedSubsets::new(
            self.items.clone(),
            self.target.clone() - self.current_sum.clone(),
        )));
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_examples() {
        let values = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let target = 20;

        let some_expected = vec![
            vec![9, 11],
            vec![1, 8, 11],
            vec![2, 7, 11],
            vec![1, 9, 10],
            vec![2, 8, 10],
            vec![3, 7, 10],
            vec![1, 4, 5, 10],
            vec![2, 3, 5, 10],
            vec![1, 2, 3, 4, 10],
            vec![3, 8, 9],
            vec![4, 7, 9],
            vec![2, 4, 5, 9],
            vec![5, 7, 8],
            vec![3, 4, 5, 8],
            vec![1, 3, 4, 5, 7],
        ];

        let results: HashSet<_> = SummedSubsets::new(values, target).collect();
        dbg!(results.len());

        for ex in some_expected {
            assert!(results.contains(&ex));
        }
    }
}
