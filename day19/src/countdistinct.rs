use std::collections::HashSet;
use std::hash::Hash;

pub trait CountDistinct: Iterator {
    fn count_distinct(self) -> usize
    where
        Self: Sized,
        Self::Item: Eq,
        Self::Item: Hash,
    {
        let mut items = HashSet::new();
        for item in self {
            items.insert(item);
        }
        items.len()
    }
}
