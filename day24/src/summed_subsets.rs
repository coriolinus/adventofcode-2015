use std::ops;
use std::fmt::Debug;

/// An iterator over subsets of the numbers `N` such that the sum of each subset is equal to a given target.
pub struct SummedSubsets<N> {
    items: Vec<N>,
    target: N,
    value: N,
    sub_iterator: Option<Box<SummedSubsets<N>>>,
}

impl<N> SummedSubsets<N> where N: Ord + Default + Debug
{
    fn default() -> SummedSubsets<N> {
        SummedSubsets {
            items: Vec::new(),
            target: N::default(),
            value: N::default(),
            sub_iterator: None,
        }
    }

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
        self.value = N::default();
        while self.value == N::default() || self.value > self.target {
            let vq = self.items.pop();
            if vq.is_some() {
                self.value = vq.unwrap();
            } else {
                // we've run out of items
                self.value = N::default();
                break;
            }
        }
    }
}

impl<N> Iterator for SummedSubsets<N> where N: Clone + Ord + Default + Debug + ops::Sub<Output = N>
{
    type Item = Vec<N>;

    fn next(&mut self) -> Option<Vec<N>> {
        if let Some(ref mut sic) = self.sub_iterator {
            // I've got a sub-iterator going; let's continue it
            // Important implication: sub-iterators only exist when self.value is some
            // sane thing, not default()
            if let Some(sir) = sic.next() {
                // my sub-iterator returned a valid subsequence
                let mut ret = sir;
                ret.push(self.value.clone());
                return Some(ret);
            }
        }

        // if we're here, either our sub-iterator has exhausted itself or we haven't constructed
        // one yet. Either way, we need to make one.
        self.sub_iterator = None;
        self.next_value();
        if self.value == N::default() {
            // we're out of items
            return None;
        }
        // If we've found a perfect match, just return now instead of bothering with the sub-iterator.
        if self.value == self.target {
            return Some(vec![self.value.clone()]);
        }

        // Ok, self.value is greater than any item in self.items, but less than self.target. This
        // is when we need a sub-iterator.
        self.sub_iterator = Some(Box::new(SummedSubsets::new(self.items.clone(),
                                                             self.target.clone() -
                                                             self.value.clone())));
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

        let some_expected = vec![vec![9, 11],
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
                                 vec![1, 3, 4, 5, 7]];



        let mut results = HashSet::new();
        results.extend(SummedSubsets::new(values, target));
        let mut rp = results.iter()
                            .map(|v| {
                                let mut v = v.clone();
                                v.reverse();
                                v
                            })
                            .collect::<Vec<_>>();
        rp.sort();

        println!("");
        println!("Results:");
        println!("{:?}", rp);
        println!("");

        // panic!("Show me the money!");

        for ex in some_expected {
            println!("Expect results to contain {:?}", ex);
            assert!(results.contains(&ex));
        }
    }
}
