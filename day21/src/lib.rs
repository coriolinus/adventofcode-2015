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


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Item {
    name: String,
    itype: ItemType,
    cost: u32,
    damage: u32,
    armor: u32,
}

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
