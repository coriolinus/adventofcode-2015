//! # Day 22: Wizard Simulator 20XX
//!
//! Little Henry Case decides that defeating bosses with swords and stuff is boring. Now he's
//! playing the game with a wizard. Of course, he gets stuck on another boss and needs your help
//! again.
//!
//! In this version, combat still proceeds with the player and the boss taking alternating turns.
//! The player still goes first. Now, however, you don't get any equipment; instead, you must
//! choose one of your spells to cast. The first character at or below `0` hit points loses.
//!
//! Since you're a wizard, you don't get to wear armor, and you can't attack normally. However,
//! since you do magic damage, your opponent's armor is ignored, and so the boss effectively has
//! zero armor as well. As before, if armor (from a spell, in this case) would reduce damage below
//! 1, it becomes 1 instead - that is, the boss' attacks always deal at least 1 damage.
//!
//! On each of your turns, you must select one of your spells to cast. If you cannot afford to
//! cast any spell, you lose. Spells cost mana; you start with 500 mana, but have no maximum limit.
//! You must have enough mana to cast a spell, and its cost is immediately deducted when you cast
//! it. Your spells are Magic Missile, Drain, Shield, Poison, and Recharge.
//!
//! - Magic Missile costs `53` mana. It instantly does `4` damage.
//! - Drain costs `73` mana. It instantly does `2` damage and heals you for `2` hit points.
//! - Shield costs `113` mana. It starts an effect that lasts for `6` turns. While it is active,
//!   your armor is increased by `7`.
//! - Poison costs `173` mana. It starts an effect that lasts for `6` turns. At the start of each
//!   turn while it is active, it deals the boss `3` damage.
//! - Recharge costs `229` mana. It starts an effect that lasts for `5` turns. At the start of
//!   each turn while it is active, it gives you `101` new mana.
//!
//!  Effects all work the same way. Effects apply at the start of both the player's turns and the
//!  boss' turns. Effects are created with a timer (the number of turns they last); at the start
//!  of each turn, after they apply any effect they have, their timer is decreased by one. If this
//!  decreases the timer to zero, the effect ends. You cannot cast a spell that would start an
//!  effect which is already active. However, effects can be started on the same turn they end.
//!
//! For example, suppose the player has `10` hit points and `250` mana, and that the boss has `13` hit
//! points and `8` damage:
//!
//! ```notrust
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 13 hit points
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 77 mana
//! - Boss has 13 hit points
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 damage.
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 77 mana
//! - Boss has 10 hit points
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 0 armor, 24 mana
//! - Boss has 3 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! Now, suppose the same initial conditions, except that the boss has `14` hit points instead:
//!
//! ```notrust
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 14 hit points
//! Player casts Recharge.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 21 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 4.
//! Boss attacks for 8 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 122 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 3.
//! Player casts Shield, increasing armor by 7.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 110 mana
//! - Boss has 14 hit points
//! Shield's timer is now 5.
//! Recharge provides 101 mana; its timer is now 2.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 211 mana
//! - Boss has 14 hit points
//! Shield's timer is now 4.
//! Recharge provides 101 mana; its timer is now 1.
//! Player casts Drain, dealing 2 damage, and healing 2 hit points.
//!
//! -- Boss turn --
//! - Player has 3 hit points, 7 armor, 239 mana
//! - Boss has 12 hit points
//! Shield's timer is now 3.
//! Recharge provides 101 mana; its timer is now 0.
//! Recharge wears off.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 7 armor, 340 mana
//! - Boss has 12 hit points
//! Shield's timer is now 2.
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 167 mana
//! - Boss has 12 hit points
//! Shield's timer is now 1.
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 167 mana
//! - Boss has 9 hit points
//! Shield's timer is now 0.
//! Shield wears off, decreasing armor by 7.
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 1 hit point, 0 armor, 114 mana
//! - Boss has 2 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! You start with 50 hit points and 500 mana points. The boss's actual stats are in your puzzle
//! input. What is the least amount of mana you can spend and still win the fight? (Do not include
//! mana recharge effects as "spending" negative mana.)

pub mod effects;
use effects::{Magic, Effects, EffectImpl};
use effects::magic_missile::MagicMissile;
use effects::drain::Drain;
use effects::shield::Shield;
use effects::poison::Poison;
use effects::recharge::Recharge;

use std::collections::VecDeque;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CharacterType {
    Player,
    Boss,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Character {
    ctype: CharacterType,
    hp: u8,
    damage: u8,
    armor: u8,
    mana: u16,
}

impl Character {
    pub fn makeboss(hp: u8, damage: u8) -> Character {
        Character {
            hp: hp,
            damage: damage,
            ..Character::boss()
        }
    }

    /// The boss, as given in the puzzle input
    pub fn boss() -> Character {
        Character {
            ctype: CharacterType::Boss,
            hp: 55,
            damage: 8,
            armor: 0,
            mana: 0,
        }
    }

    pub fn makeplayer(hp: u8, mana: u16) -> Character {
        Character {
            hp: hp,
            mana: mana,
            ..Character::player()
        }
    }

    pub fn player() -> Character {
        Character {
            ctype: CharacterType::Player,
            hp: 50,
            damage: 0,
            armor: 0,
            mana: 500,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Arena {
    turn: CharacterType,
    player: Character,
    boss: Character,
    effects: Vec<EffectImpl>,
    pub mana_spent: u16,
    last_spell: Option<Effects>,
    log: String,
    turn_log: String,
}

impl Default for Arena {
    fn default() -> Arena {
        Arena {
            turn: CharacterType::Player,
            player: Character::player(),
            boss: Character::boss(),
            effects: Vec::new(),
            mana_spent: 0,
            last_spell: None,
            log: String::new(),
            turn_log: String::new(),
        }
    }
}

impl Arena {
    pub fn new(player: Character, boss: Character) -> Arena {
        Arena {
            player: player,
            boss: boss,
            ..Arena::default()
        }
    }

    fn future(&self) -> Arena {
        let mut ret = self.clone();
        ret.last_spell = None;
        ret.turn = match ret.turn {
            CharacterType::Player => CharacterType::Boss,
            CharacterType::Boss => CharacterType::Player,
        };
        ret
    }

    fn attempt_spell(&self, spell: &Magic) -> Option<Arena> {
        if self.player.mana >= spell.cost() {
            // You cannot cast a spell that would start an effect which is already active.
            // However, effects can be started on the same turn they end.
            for eff in &self.effects {
                if eff.etype == spell.etype() && eff.ttl > 0 {
                    return None;
                }
            }

            let mut future = self.future();
            future.last_spell = Some(spell.etype());
            future.mana_spent += spell.cost();
            spell.on_cast(&mut future.player, &mut future.boss);
            future.turn_log.push_str(&spell.on_cast_str());
            if spell.ttl() > 0 {
                future.effects.push(spell.to_impl());
            }
            Some(future)
        } else {
            None
        }
    }

    pub fn log(&self) -> String {
        let mut ret = self.log.clone();
        ret.push_str(&self.turn_log);
        ret
    }

    /// Take one turn.
    ///
    /// Returns Ok(vector of future Arenas) if the game should continue.
    /// Returns Err(victor) if the game should end.
    ///
    /// Game should end if either character runs out of hit points or the player character
    /// has insufficient mana to cast any spell on their turn.
    pub fn turn(&mut self) -> Result<Vec<Arena>, CharacterType> {
        // did the player win last turn?
        if self.boss.hp == 0 {
            self.turn_log.push_str("This kills the boss, and the player wins.\n");
            return Err(CharacterType::Player);
        }

        self.log.push_str(&self.turn_log);
        self.log.push('\n');

        self.turn_log = format!("-- {:?} turn --\n", self.turn);
        let mut line = format!("- Player has {} hit points, {} armor, {} mana\n", self.player.hp, self.player.armor, self.player.mana);
        self.turn_log.push_str(&line);
        line = format!("- Boss has {} hit points\n", self.boss.hp);
        self.turn_log.push_str(&line);

        // buffer for next turn's effects
        let mut nte = Vec::new();
        // Effects apply at the start of each player's turn.
        for effectimpl in &self.effects {
            let ei = effectimpl.etype.clone();
            let mut effect : Box<Magic> = match ei {
                Effects::Drain => Box::new(Drain::from_ei(effectimpl.clone())),
                Effects::MagicMissile => Box::new(MagicMissile::from_ei(effectimpl.clone())),
                Effects::Poison => Box::new(Poison::from_ei(effectimpl.clone())),
                Effects::Recharge => Box::new(Recharge::from_ei(effectimpl.clone())),
                Effects::Shield => Box::new(Shield::from_ei(effectimpl.clone())),
            };

            effect.per_turn(&mut self.player, &mut self.boss);
            self.turn_log.push_str(&effect.per_turn_str());

            if effect.ttl() > 0 {
                nte.push(effect.to_impl());
            }
        }
        // After application, remove those who are out of life.
        self.effects = nte;

        // has the player won yet?
        if self.boss.hp == 0 {
            self.turn_log.push_str("This kills the boss, and the player wins.\n");
            return Err(CharacterType::Player);
        }

        match self.turn {
            CharacterType::Boss => {
                    let damage = if self.boss.damage > self.player.armor {self.boss.damage - self.player.armor} else {1};
                    self.turn_log.push_str(&format!("Boss attacks for {} - {} = {} damage!\n", self.boss.damage, self.player.armor, damage));
                    if self.player.hp > damage {
                        self.player.hp -= damage;
                        let mut ret = self.clone();
                        ret.turn = CharacterType::Player;
                        Ok(vec![self.future()])
                    } else {
                        // damage >= self.player.hp
                        self.turn_log.push_str("This kills the player, and the boss wins.\n");
                        self.player.hp = 0;
                        Err(CharacterType::Boss)
                    }

            },
            CharacterType::Player => {
                if self.player.hp > 0 {
                    // For each spell we can cast, add a future in which we cast it
                    let mut ret = Vec::new();

                    // sorted from low mana to high, for correct results
                    let spells: Vec<Box<Magic>> = vec![Box::new(MagicMissile::new()),
                                                       Box::new(Drain::new()),
                                                       Box::new(Shield::new()),
                                                       Box::new(Poison::new()),
                                                       Box::new(Recharge::new())];
                    for spell in spells {
                        if let Some(future) = self.attempt_spell(&*spell) {
                            ret.push(future)
                        }
                    }

                    match ret.len() {
                        0 => Err(CharacterType::Boss),
                        _ => Ok(ret),
                    }
                } else {
                    Err(CharacterType::Boss)
                }
            },
        }
    }

    pub fn hard_turn(&mut self) -> Result<Vec<Arena>, CharacterType> {
        if self.turn == CharacterType::Player {
            self.player.hp -= 1;
            if self.player.hp == 0 {
                return Err(CharacterType::Boss);
            }
        }
        self.turn()
    }
}

pub fn breadth_first_victory_search(arena: Arena) -> Arena {
    breadth_first_victory_search_with_difficulty(arena, false)
}

pub fn breadth_first_victory_search_with_difficulty(arena: Arena, hard: bool) -> Arena {
    let mut found_victory = false;
    let mut candidates = Vec::new();
    let mut buffer = VecDeque::new();
    buffer.push_back(arena);
    while !buffer.is_empty() {
        let mut arena = buffer.pop_front().unwrap();
        match if hard {arena.hard_turn()} else {arena.turn()} {
            Ok(futures) => {
                if ! found_victory {
                    buffer.extend(futures);
                }
            },
            Err(victor) => {
                if victor == CharacterType::Player {
                    found_victory = true;
                    candidates.push(arena);
                }
            }
        }
    }
    candidates.iter().fold(None, |acc,  c| match acc {
        None => Some(c),
        Some(oc) => Some(if oc.mana_spent <= c.mana_spent {oc} else {c}),
    }).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::effects::Effects;

    fn expect_spell(oarena: Option<Arena>, spell: Effects) -> Option<Arena> {
        match oarena {
            None => None,
            Some(mut arena) => {
                let res = arena.turn();
                match res {
                    Err(_) => None,
                    Ok(futures) => {
                        for f in futures {
                            if f.last_spell == Some(spell.clone()) {
                                return Some(f);
                            }
                        }
                        None
                    },
                }
            }
        }
    }

    fn boss_turn(oarena: Option<Arena>) -> Option<Arena> {
        match oarena {
            None => None,
            Some(mut arena) => {
                let res = arena.turn();
                match res {
                    Err(_) => None,
                    Ok(futures) => {
                        match futures.len() {
                            1 => Some(futures[0].clone()),
                            _ => None,
                        }
                    }
                }
            }
        }
    }

    fn expect_turn(arena: &Option<Arena>, turn: CharacterType, player_hp: u8, player_armor: u8, player_mana: u16, boss_hp: u8) {
        assert!(arena.is_some());
        let arena = arena.clone().unwrap();
        assert_eq!(arena.turn, turn);
        assert_eq!(arena.player.hp, player_hp);
        assert_eq!(arena.player.armor, player_armor);
        assert_eq!(arena.player.mana, player_mana);
        assert_eq!(arena.boss.hp, boss_hp);
    }

    fn expect_victor(oarena: Option<Arena>, victor: CharacterType) {
        assert!(oarena.is_some());
        let mut arena = oarena.unwrap();
        let res = arena.turn();
        if let Err(victor_f) = res {
            assert_eq!(victor, victor_f);
            println!("{}", arena.log);
            println!("{}", arena.turn_log);
        } else {
            panic!("Didn't find victor {:?} when expected", victor);
        }
    }

    #[test]
    fn test_first_example() {
        let player = Character::makeplayer(10, 250);
        let boss = Character::makeboss(13, 8);

        let pt = CharacterType::Player;
        let bt = CharacterType::Boss;

        let mut arena = Some(Arena::new(player, boss));
        expect_turn(&arena, pt, 10, 0, 250, 13);
        arena = expect_spell(arena, Effects::Poison);
        expect_turn(&arena, bt, 10, 0, 77, 13);
        arena = boss_turn(arena);
        expect_turn(&arena, pt, 2, 0, 77, 10);
        arena = expect_spell(arena, Effects::MagicMissile);
        expect_victor(arena, pt);
    }

    #[test]
    fn test_second_example() {
        let player = Character::makeplayer(10, 250);
        let boss = Character::makeboss(14, 8);

        let pt = CharacterType::Player;
        let bt = CharacterType::Boss;

        let mut arena = Some(Arena::new(player, boss));
        expect_turn(&arena, pt, 10, 0, 250, 14);
        arena = expect_spell(arena, Effects::Recharge);
        expect_turn(&arena, bt, 10, 0, 21, 14);
        arena = boss_turn(arena);

        expect_turn(&arena, pt, 2, 0, 122, 14);
        arena = expect_spell(arena, Effects::Shield);
        expect_turn(&arena, bt, 2, 7, 110, 14);
        arena = boss_turn(arena);

        expect_turn(&arena, pt, 1, 7, 211, 14);
        arena = expect_spell(arena, Effects::Drain);
        expect_turn(&arena, bt, 3, 7, 239, 12);
        arena = boss_turn(arena);

        expect_turn(&arena, pt, 2, 7, 340, 12);
        arena = expect_spell(arena, Effects::Poison);
        expect_turn(&arena, bt, 2, 7, 167, 12);
        arena = boss_turn(arena);

        expect_turn(&arena, pt, 1, 7, 167, 9);
        arena = expect_spell(arena, Effects::MagicMissile);
        expect_victor(arena, pt);
    }
}
