use crate::{
    items::{Item, ItemType},
    loadout::Loadout,
};

/// Increment an item index. Returns `true` if it rolled over and is now `None`, otherwise `false`.
fn increment_item(item: &mut Option<usize>, collection: &[Item]) -> bool {
    match item {
        None => {
            if !collection.is_empty() {
                *item = Some(0);
            }
        }
        Some(mut current_idx) => {
            current_idx += 1;
            if current_idx < collection.len() {
                *item = Some(current_idx);
            } else {
                *item = None;
            }
        }
    }
    item.is_none()
}

#[derive(Clone)]
pub struct LoadoutGenerator {
    current: Loadout,
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
    weapon_index: usize,
    armor_index: Option<usize>,
    ring_l_index: Option<usize>,
    ring_r_index: Option<usize>,
    first_call: bool,
}

impl LoadoutGenerator {
    pub fn new(items: &[Item]) -> LoadoutGenerator {
        let mut weapons = Vec::new();
        let mut armors = Vec::new();
        let mut rings = Vec::new();

        for item in items {
            match item.itype {
                ItemType::Weapon => weapons.push(*item),
                ItemType::Armor => armors.push(*item),
                ItemType::Ring => rings.push(*item),
            }
        }

        let dagger = *weapons.first().unwrap();
        let current = Loadout::new(dagger, None, None, None).unwrap();

        LoadoutGenerator {
            current,
            weapons,
            armors,
            rings,
            weapon_index: 0,
            armor_index: None,
            ring_l_index: None,
            ring_r_index: None,
            first_call: true,
        }
    }

    fn as_loadout(&self) -> Loadout {
        Loadout {
            weapon: self.weapons[self.weapon_index],
            armor: self.armor_index.map(|idx| self.armors[idx]),
            ring_l: self.ring_l_index.map(|idx| self.rings[idx]),
            ring_r: self.ring_r_index.map(|idx| self.rings[idx]),
        }
    }

    /// Increment the armor index. Return True if it rolled over and is now None, otherwise False.
    fn increment_armor(&mut self) -> bool {
        increment_item(&mut self.armor_index, &self.armors)
    }

    /// Increment the right ring index. Return True if it rolled over and is now None, otherwise False.
    fn increment_ring_r(&mut self) -> bool {
        increment_item(&mut self.ring_r_index, &self.rings)
    }

    /// Increment the left ring index. Return True if it rolled over and is now None, otherwise False.
    fn increment_ring_l(&mut self) -> bool {
        increment_item(&mut self.ring_l_index, &self.rings)
    }
}

// This implementation looks _sketchy_. A reasonable target for refactor, later.
impl Iterator for LoadoutGenerator {
    type Item = Loadout;
    /// Generate the next loadout.
    ///
    /// Ordering is arbitrary.
    fn next(&mut self) -> Option<Loadout> {
        // we have to have a weapon
        if self.weapons.is_empty() {
            return None;
        }

        if self.first_call {
            self.first_call = false;
            return Some(self.as_loadout());
        }

        // Time to slowly tune everything up.
        if self.weapon_index < self.weapons.len() - 1 {
            // increment the weapon
            self.weapon_index += 1;
        } else {
            self.weapon_index = 0;
            if self.increment_armor() {
                // armor rolled over
                if self.increment_ring_r() {
                    // ring r rolled over
                    if self.increment_ring_l() {
                        // ring l rolled over
                        // That's it! That's the end of the iteration!
                        return None;
                    }
                    // ring r is None, but that's not right. It is always higher than ring l.
                    // also, if we're here, ring l is not None.
                    let slri = self.ring_l_index.clone().unwrap();
                    if slri < self.rings.len() - 1 {
                        self.ring_r_index = Some(slri + 1);
                    } else {
                        // this should never happen, because it would break the condition
                        // that self.ring_r_index is always > self.ring_l_index.
                        //
                        // however, it does in fact happen because we can't guard against that
                        // in self.increment_ring_l because when that function is called,
                        // self.ring_r_index is guaranteed to be None.
                        //
                        // How about, instead of panicing, we just recurse a little and skip this
                        // result?
                        return self.next();
                    }
                }
            }
        }
        Some(self.as_loadout())
    }
}
