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

    pub fn upgrade_weapon(&mut self, weapons_list: &Vec<Item>) -> bool {
        let mut next_one = false;
        for weapon in weapons_list {
            if next_one {
                self.weapon = weapon.clone();
                // also has the side effect of eliminating the rest of the loadout
                self.armor = None;
                self.ring_l = None;
                self.ring_r = None;

                return true;
            } else if self.weapon == *weapon {
                next_one = true;
            }
        }
        false
    }

    pub fn upgrade_armor(&mut self, armors_list: &Vec<Item>) -> bool {
        if self.armor.is_none() {
            self.armor = match armors_list.first() {
                None => None,
                Some(armor) => Some(armor.clone()),
            };
            return true;
        }

        let mut new_armor = None;
        if let Some(ref sarmor) = self.armor {
            let mut next_one = false;
            for armor in armors_list {
                if next_one {
                    new_armor = Some(armor.clone());
                    break;
                } else if armor == sarmor {
                    next_one = true;
                }
            }
        }
        if let Some(armor) = new_armor {
            // we found a replacement
            self.armor = Some(armor);
            return true;
        }

        false
    }

    fn find_next_ring(&mut self, ring: &Item, rings_list: &Vec<Item>) -> Option<Item> {
        let mut next_ring = None;
        let mut next_one = false;
        for lring in rings_list {
            if next_one {
                next_ring = Some(ring.clone());
                break;
            } else if lring == ring {
                next_one = true;
            }
        }
        next_ring
    }

    pub fn upgrade_rings(&mut self, rings_list: &Vec<Item>) -> bool {
        if rings_list.len() == 0 {
            return false;
        } else if rings_list.len() == 1 && self.ring_r.is_some() {
            // the only ring in the list is already assigned
            return false;
        } else {
            let ref second_last = rings_list[rings_list.len() - 2];
            let ref last = rings_list[rings_list.len() - 1];

            if self.ring_l == Some(second_last.to_owned()) && self.ring_r == Some(last.to_owned()) {
                // our rings are already the last two
                return false;
            }
        }

        if self.ring_r.is_none() {
            self.ring_r = match rings_list.first() {
                None => None,
                Some(ring) => Some(ring.clone()),
            };
            return true;
        }

        let srr = self.ring_r.clone().unwrap();
        let upgrade_right = self.find_next_ring(&srr, rings_list); // if None, we're out of rings on the right.
        if upgrade_right.is_none() {
            // we can't upgrade the right side, so let's upgrade the left.
            if self.ring_l.is_none() {
                self.ring_l = match rings_list.first() {
                    None => None,
                    Some(ring) => Some(ring.clone()),
                };
                return true;
            } else {
                let srl = self.ring_l.clone().unwrap();
                let upgrade_left = self.find_next_ring(&srl, rings_list);
                if let Some(ring) = upgrade_left {
                    self.ring_l = Some(ring);
                    return true;
                }
                return false;
            }
        }

        // upgrade_right is Some(ring)
        let upgrade_right = upgrade_right.unwrap();

        // let's figure out the cost of upgrading the left ring:
        // It's the cost of the next left ring plus the cost of the current right ring
        let upgrade_left = if self.ring_l.is_none() {
                               match rings_list.first() {
                                   None => None,
                                   Some(ring) => Some(ring.clone()),
                               }
                           } else {
                               let srl = self.ring_l.clone().unwrap();
                               self.find_next_ring(&srl, rings_list)
                           }
                           .unwrap();

        let upgrade_left_cost = srr.cost + upgrade_left.cost;

        // we have two fundamental options: upgrade the right ring while removing the left, or
        // upgrade the left ring. The choice depends on two factors. If upgrading the left ring
        // would make it the same as the right, we upgrade right and reset. Otherwise, we simply
        // choose the one with the lower total ring value.

        if upgrade_left == srr || upgrade_right.cost <= upgrade_left_cost {
            // upgrade the right ring and take off the left
            self.ring_l = None;
            self.ring_r = Some(upgrade_right);
            true
        } else {
            // upgrade the left ring
            self.ring_l = Some(upgrade_left);
            true
        }
    }
}

#[derive(Clone)]
pub struct LoadoutGenerator {
    current: Loadout,
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
    first_call: bool,
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

        let dagger = w.first().unwrap().clone();
        let initial = Loadout::new(dagger, None, None, None).unwrap();

        LoadoutGenerator {
            current: initial,
            weapons: w,
            armors: a,
            rings: r,
            first_call: true,
        }
    }
}

impl Iterator for LoadoutGenerator {
    type Item = Loadout;

    fn next(&mut self) -> Option<Loadout> {
        if self.first_call {
            self.first_call = false;
            return Some(self.current.clone());
        }
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

        // we optimize by noting that as we always need a weapon, but do not require anything else,
        // every time we upgrade a weapon, the cheapest thing is to throw away all
        // other equipment.

        let mut wpn_up = self.current.clone();
        let wpn_up_worked = wpn_up.upgrade_weapon(&self.weapons);
        let mut arm_up = self.current.clone();
        let arm_up_worked = arm_up.upgrade_armor(&self.armors);
        let mut rng_up = self.current.clone();
        let rng_up_worked = rng_up.upgrade_rings(&self.rings);

        if !(wpn_up_worked || arm_up_worked || rng_up_worked) {
            return None;
        }

        let mut costs = Vec::new();
        if wpn_up_worked {
            costs.push(wpn_up.cost());
        }
        if arm_up_worked {
            costs.push(arm_up.cost());
        }
        if rng_up_worked {
            costs.push(rng_up.cost());
        }

        let min_cost = costs.iter().min().unwrap();

        if rng_up_worked && &rng_up.cost() == min_cost {
            // improve our rings
            self.current = rng_up.clone();
            return Some(rng_up);
        }


        if arm_up_worked && &arm_up.cost() == min_cost {
            // improve our armor
            self.current = arm_up.clone();
            return Some(arm_up);
        }


        if wpn_up_worked && &wpn_up.cost() == min_cost {
            self.current = wpn_up.clone();
            return Some(wpn_up);
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

    #[test]
    fn test_loadout_generator() {
        let lg = LoadoutGenerator::new();
        unimplemented!()
    }
}
