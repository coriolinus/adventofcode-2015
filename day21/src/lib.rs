//! # Day 21: RPG Simulator 20XX
//!
//! Little Henry Case got a new video game for Christmas. It's an RPG, and he's stuck on a boss.
//! He needs to know what equipment to buy at the shop. He hands you the controller.
//!
//! In this game, the player (you) and the enemy (the boss) take turns attacking. The player
//! always goes first. Each attack reduces the opponent's hit points by at least `1`. The first
//! character at or below 0 hit points loses.
//!
//! Damage dealt by an attacker each turn is equal to the attacker's damage score minus the
//! defender's armor score. An attacker always does at least 1 damage. So, if the attacker has a
//! damage score of 8, and the defender has an armor score of 3, the defender loses 5 hit points.
//! If the defender had an armor score of 300, the defender would still lose 1 hit point.
//!
//! Your damage score and armor score both start at zero. They can be increased by buying items in
//! exchange for gold. You start with no items and have as much gold as you need. Your total damage
//! or armor is equal to the sum of those stats from all of your items. You have 100 hit points.
//!
//! Here is what the item shop is selling:
//!
//! ```notrust
//! Weapons:    Cost  Damage  Armor
//! Dagger        8     4       0
//! Shortsword   10     5       0
//! Warhammer    25     6       0
//! Longsword    40     7       0
//! Greataxe     74     8       0
//!
//! Armor:      Cost  Damage  Armor
//! Leather      13     0       1
//! Chainmail    31     0       2
//! Splintmail   53     0       3
//! Bandedmail   75     0       4
//! Platemail   102     0       5
//!
//! Rings:      Cost  Damage  Armor
//! Damage +1    25     1       0
//! Damage +2    50     2       0
//! Damage +3   100     3       0
//! Defense +1   20     0       1
//! Defense +2   40     0       2
//! Defense +3   80     0       3
//! ```
//!
//! You must buy exactly one weapon; no dual-wielding. Armor is optional, but you can't use more
//! than one. You can buy 0-2 rings (at most one for each hand). You must use any items you buy.
//! The shop only has one of each item, so you can't buy, for example, two rings of Damage +3.
//!
//! For example, suppose you have 8 hit points, 5 damage, and 5 armor, and that the boss has 12 hit
//! points, 7 damage, and 2 armor:
//!
//! - The player deals `5-2 = 3` damage; the boss goes down to `9` hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to `6` hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to `6` hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to `4` hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to `3` hit points.
//! - The boss deals `7-5 = 2` damage; the player goes down to `2` hit points.
//! - The player deals `5-2 = 3` damage; the boss goes down to `0` hit points.
//!
//! In this scenario, the player wins! (Barely.)
//!
//! You have `100` hit points. The boss's actual stats are in your puzzle input. What is the least
//! amount of gold you can spend and still win the fight?


#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct Item {
    name: String,
    itype: ItemType,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn bare(itype: ItemType) -> Item {
        Item {
            name: "".to_string(),
            itype: itype,
            cost: 0,
            damage: 0,
            armor: 0,
        }
    }

    fn weapon(name: &str, cost: u32, damage: u32) -> Item {
        Item {
            name: name.to_string(),
            damage: damage,
            cost: cost,
            ..Item::bare(ItemType::Weapon)
        }
    }

    fn armor(name: &str, cost: u32, armor: u32) -> Item {
        Item {
            name: name.to_string(),
            armor: armor,
            cost: cost,
            ..Item::bare(ItemType::Armor)
        }
    }

    fn ring(name: &str, cost: u32, damage: u32, armor: u32) -> Item {
        Item {
            name: name.to_string(),
            damage: damage,
            armor: armor,
            cost: cost,
            ..Item::bare(ItemType::Ring)
        }
    }
}

pub fn item_shop() -> Vec<Item> {
    let mut ret = Vec::new();
    // weapons
    ret.push(Item::weapon("Dagger", 8, 4));
    ret.push(Item::weapon("Shortsword", 10, 5));
    ret.push(Item::weapon("Warhammer", 25, 6));
    ret.push(Item::weapon("Longsword", 40, 7));
    ret.push(Item::weapon("Greataxe", 74, 8));

    // armor
    ret.push(Item::armor("Leather", 13, 1));
    ret.push(Item::armor("Chainmail", 31, 2));
    ret.push(Item::armor("Splintmail", 53, 3));
    ret.push(Item::armor("Bandedmail", 75, 4));
    ret.push(Item::armor("Platemail", 102, 5));

    // rings
    ret.push(Item::ring("Defense +1", 20, 0, 1));
    ret.push(Item::ring("Damage +1", 25, 1, 0));
    ret.push(Item::ring("Defense +2", 40, 0, 2));
    ret.push(Item::ring("Damage +2", 50, 2, 0));
    ret.push(Item::ring("Defense +3", 80, 0, 3));
    ret.push(Item::ring("Damage +3", 100, 3, 0));

    ret
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    ring_l: Option<Item>,
    ring_r: Option<Item>,
}

impl Loadout {
    pub fn new(weapon: Item,
               armor: Option<Item>,
               ring_l: Option<Item>,
               ring_r: Option<Item>)
               -> Option<Loadout> {
        if weapon.itype != ItemType::Weapon {
            return None;
        }
        if armor.is_some() && armor.clone().unwrap().itype != ItemType::Armor {
            return None;
        }
        if ring_l.is_some() && ring_l.clone().unwrap().itype != ItemType::Ring {
            return None;
        }
        if ring_r.is_some() && ring_r.clone().unwrap().itype != ItemType::Ring {
            return None;
        }
        Some(Loadout {
            weapon: weapon,
            armor: armor,
            ring_l: ring_l,
            ring_r: ring_r,
        })
    }

    pub fn as_vec(&self) -> Vec<Item> {
        vec![&Some(self.weapon.clone()), &self.armor, &self.ring_l, &self.ring_r]
            .iter()
            .filter(|ref i| i.is_some())
            .cloned()
            .map(|i| i.clone().unwrap())
            .collect()
    }

    pub fn cost(&self) -> u32 {
        self.as_vec().iter().fold(0, |acc, ref item| acc + item.cost)
    }

    pub fn damage(&self) -> u32 {
        self.as_vec().iter().fold(0, |acc, ref item| acc + item.damage)
    }

    pub fn armor(&self) -> u32 {
        self.as_vec().iter().fold(0, |acc, ref item| acc + item.armor)
    }
}

#[derive(Clone)]
pub struct LoadoutGenerator {
    current: Loadout,
    weapons: Vec<Item>,
    armors: Vec<Item>,
    armor: Option<usize>,
    rings: Vec<Item>,
    ring_l: Option<usize>,
    ring_r: Option<usize>,
}

impl LoadoutGenerator {
    pub fn new() -> LoadoutGenerator {
        let mut w = Vec::new();
        let mut a = Vec::new();
        let mut r = Vec::new();

        for item in item_shop() {
            match item.itype {
                ItemType::Weapon => w.push(item),
                ItemType::Armor => a.push(item),
                ItemType::Ring => r.push(item),
            }
        }

        w.reverse();
        // don't reverse the armor because we also access it by index
        // a.reverse();
        // don't reverse the rings because we access them differently.
        // r.reverse();

        // now we can always get the next item with a simple `.pop()`
        let dagger = w.pop().unwrap();
        let initial = Loadout::new(dagger, None, None, None).unwrap();

        LoadoutGenerator {
            current: initial,
            weapons: w,
            armors: a,
            rings: r,
            ring_l: None,
            ring_r: None,
            armor: None,
        }
    }

    fn upgrade_weapon(&self) -> Option<(Loadout, Vec<Item>)> {
        let mut weapons = self.weapons.clone();
        match weapons.pop() {
            None => None,
            Some(weapon) => {
                let mut ret = self.current.clone();
                ret.weapon = weapon.clone();
                ret.armor = None;
                ret.ring_l = None;
                ret.ring_r = None;
                Some((ret, weapons))
            }
        }
    }

    fn upgrade_armor(&self) -> Option<(Loadout, Option<usize>)> {
        if self.armors.len() == 0 {
            return None;
        }
        let mut ret = self.current.clone();
        if self.armor.is_none() {
            ret.armor = Some(self.armors[0].clone());
            return Some((ret, Some(0)));
        }
        if let Some(sa) = self.armor {
            if sa == self.armors.len() - 1 {
                return None;
            } else {
                ret.armor = Some(self.armors[sa + 1].clone());
                return Some((ret, Some(sa + 1)));
            }
        }
        None
    }

    fn upgrade_rings(&self) -> Option<(Loadout, Option<usize>, Option<usize>)> {
        if self.rings.len() == 0 {
            return None;
        } else if self.rings.len() == 1 && self.ring_r == Some(0) {
            return None;
        } else if self.ring_r == Some(self.rings.len() - 1) && self.ring_l == Some(self.rings.len() - 2) {
            return None;
        }

        let mut ret = self.current.clone();
        if self.ring_r.is_none() {
            ret.ring_r = Some(self.rings[0].clone());
            return Some((ret, None, Some(0)));
        }

        let srr = self.ring_r.to_owned().unwrap();

        if srr == self.rings.len() - 1 {
            if self.ring_l.is_none() {
                ret.ring_l = Some(self.rings[0].clone());
                return Some((ret, Some(0), Some(srr)));
            } else {
                // only increase ring_l because ring_r is already maxed out
                let srl = self.ring_l.to_owned().unwrap();
                ret.ring_l = Some(self.rings[srl + 1].clone());
                return Some((ret, Some(srl), Some(srr)));
            }
        }

        // srr is not None or max
        // we have to choose between two options:
        //  - increment left
        //  - increment right, take off left
        let inc_right = self.rings[srr + 1].cost - self.rings[srr].cost;
        let inc_left = if self.ring_l.is_none() {
            self.rings[0].cost
        } else {
            let srl = self.ring_l.to_owned().unwrap();
            self.rings[srl + 1].cost - self.rings[srl].cost
        };

        if (self.ring_l.is_some() && self.ring_l.to_owned().unwrap() == srr - 1) ||
           inc_right < inc_left {
            // increase the ring on the right side and remove the left ring
            ret.ring_l = None;
            ret.ring_r = Some(self.rings[srr + 1].clone());
            Some((ret, None, Some(srr + 1)))
        } else {
            // just increment the left ring
            let srl = self.ring_l.to_owned().unwrap();
            ret.ring_l = Some(self.rings[srl + 1].clone());
            Some((ret, Some(srl), Some(srr)))
        }
    }
}

impl Iterator for LoadoutGenerator {
    type Item = Loadout;

    fn next(&mut self) -> Option<Loadout> {
        // The goal here is to always return the cheapest equipment upgrade, even if that means
        // downgrading one piece of equipment, possibly more than once, in order to upgrade
        // something else.
        //
        // At the same time, we want to iterate deterministically through the items and return
        // each possible loadout only once.
        //
        // As an implementation detail to help reduce the options applied, we determine that the
        // ring on the right hand is always more valuable than the ring on the left.

        // start by figuring the costs of upgrade of each category: weapon, armor, ring

        let wpn_up = self.upgrade_weapon();
        let arm_up = self.upgrade_armor();
        let rng_up = self.upgrade_rings();

        if wpn_up == None && arm_up == None && rng_up == None {
            return None;
        }

        let mut costs = Vec::new();
        if let Some((ref wpns, _)) = wpn_up {
            costs.push(wpns.cost());
        }
        if let Some((ref arms, _)) = arm_up {
            costs.push(arms.cost());
        }
        if let Some((ref rngs, _, _)) = rng_up {
            costs.push(rngs.cost());
        }

        let min_cost = costs.iter()
                            .min()
                            .unwrap()
                            .clone();

        if let Some((loadout, left_ring_index, right_ring_index)) = rng_up {
            if loadout.cost() == min_cost {
                // improve our rings
                self.ring_l = left_ring_index;
                self.ring_r = right_ring_index;
                self.current = loadout.clone();
                return Some(loadout);
            }
        }

        if let Some((loadout, armor_index)) = arm_up {
            if loadout.cost() == min_cost {
                // improve our armor
                self.armor = armor_index;
                self.current = loadout.clone();
                return Some(loadout);
            }
        }

        if let Some((loadout, wpns)) = wpn_up {
            if loadout.cost() == min_cost {
                // improve our weapons
                self.weapons = wpns;
                self.current = loadout.clone();
                // reset the other fields
                self.ring_l = None;
                self.ring_r = None;
                self.armor = None;

                return Some(loadout);
            }
        }

        panic!("Failed to calculate min costs correctly in LoadoutGenerator.next()");
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CharacterType {
    Player,
    Boss,
}

pub struct Character {
    ctype: CharacterType,
    hp: u32,
    damage: u32,
    armor: u32,
}


impl Character {
    /// The boss, as given in the puzzle input
    pub fn boss() -> Character {
        Character {
            ctype: CharacterType::Boss,
            hp: 104,
            damage: 8,
            armor: 1,
        }
    }

    pub fn player(loadout: &Loadout) -> Character {
        Character {
            ctype: CharacterType::Player,
            hp: 100,
            damage: loadout.damage(),
            armor: loadout.armor(),
        }
    }
}

pub fn combat(player: Character, boss: Character) -> Character {
    let mut agent = player;
    let mut respondent = boss;

    loop {
        // calc damage
        let mut damage = agent.damage - respondent.armor;
        if damage < 1 {
            damage = 1;
        }
        // apply
        respondent.hp -= damage;
        if respondent.hp <= 0 {
            return agent;
        }
        // swap roles
        let temp = agent;
        agent = respondent;
        respondent = temp;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_combat() {
        let player = Character {
            ctype: CharacterType::Player,
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            ctype: CharacterType::Boss,
            hp: 12,
            damage: 7,
            armor: 2,
        };
        let winner = combat(player, boss);
        assert_eq!(winner.ctype, CharacterType::Player);
        assert_eq!(winner.hp, 2);
    }
}
