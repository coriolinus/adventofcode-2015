use crate::{
    items::{Item, ItemType},
    loadout::Loadout,
};
use itertools::Itertools;

/// Produce `None`, followed by `Some(t)` for each item in `ts`.
fn optional_iter<T: Clone>(
    ts: impl Iterator<Item = T> + Clone,
) -> impl Iterator<Item = Option<T>> + Clone {
    std::iter::once(None).chain(ts.map(Some))
}

/// Produce an iterator over sets of 0, 1, 2 distinct rings.
fn rings_iter<T: Ord + Copy>(
    rings: impl Iterator<Item = T> + Clone,
) -> impl Iterator<Item = (Option<T>, Option<T>)> + Clone {
    optional_iter(rings.clone())
        .cartesian_product(optional_iter(rings))
        .filter(|rings| match rings {
            // given two rings, they must be distinct, and the more powerful must be on the right
            // (for efficiency)
            (Some(left), Some(right)) => left < right,
            // discard left-hand-only cases for efficiency (redundant with right-hand-only)
            (Some(_), None) => false,
            // consider all the other cases
            _ => true,
        })
}

pub fn loadout_generator(items: &[Item]) -> impl '_ + Iterator<Item = Loadout> {
    let filter_items =
        |item_type: ItemType| items.iter().filter(move |item| item.itype == item_type);
    let weapons = filter_items(ItemType::Weapon);
    let armors = filter_items(ItemType::Armor);
    let rings = filter_items(ItemType::Ring);

    weapons
        .into_iter()
        .cartesian_product(optional_iter(armors))
        .cartesian_product(rings_iter(rings.copied()))
        .map(|((weapon, armor), (left_ring, right_ring))| Loadout {
            weapon: *weapon,
            armor: armor.copied(),
            ring_l: left_ring,
            ring_r: right_ring,
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn rings_iter_produces_correct_items() {
        let rings: HashSet<_> = rings_iter(0..3).collect();
        println!("{:#?}", rings);
        assert_eq!(
            rings,
            hashset! {
                (None, None),
                (None, Some(0)),
                (None, Some(1)),
                (None, Some(2)),
                (Some(0), Some(1)),
                (Some(0), Some(2)),
                (Some(1), Some(2)),
            }
        );
    }
}
