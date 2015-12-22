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
    mana_spent: u16,
}

impl Default for Arena {
    fn default() -> Arena {
        Arena {
            turn: CharacterType::Player,
            player: Character::player(),
            boss: Character::boss(),
            effects: Vec::new(),
            mana_spent: 0,
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
        ret.turn = match ret.turn {
            CharacterType::Player => CharacterType::Boss,
            CharacterType::Boss => CharacterType::Player,
        };
        ret
    }

    fn attempt_spell(&self, spell: &Magic) -> Option<Arena> {
        if self.player.mana >= spell.cost() {
            let mut future = self.future();
            future.mana_spent += spell.cost();
            spell.on_cast(&mut future.player, &mut future.boss);
            if spell.ttl() > 0 {
                future.effects.push(spell.to_impl());
            }
            Some(future)
        } else {
            None
        }
    }

    /// Take one turn.
    ///
    /// Returns Ok(vector of future Arenas) if the game should continue.
    /// Returns Err(victor) if the game should end.
    ///
    /// Game should end if either character runs out of hit points or the player character
    /// has insufficient mana to cast any spell on their turn.
    pub fn turn(&mut self) -> Result<Vec<Arena>, CharacterType> {
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
        }
        // After application, remove those who are out of life.
        self.effects.retain(|ef| ef.ttl > 0);

        match self.turn {
            CharacterType::Boss => {
                if self.boss.hp > 0 {
                    let damage = if self.boss.damage > self.player.armor {self.boss.damage - self.player.armor} else {1};
                    if self.player.hp > damage {
                        self.player.hp -= damage;
                        let mut ret = self.clone();
                        ret.turn = CharacterType::Player;
                        Ok(vec![self.future()])
                    } else {
                        // damage >= self.player.hp
                        self.player.hp = 0;
                        Err(CharacterType::Boss)
                    }
                } else {
                    // boss died, turn ends without switching character.
                    Err(CharacterType::Player)
                }
            },
            CharacterType::Player => {
                if self.player.hp > 0 {
                    // For each spell we can cast, add a future in which we cast it
                    let mut ret = Vec::new();
                    //Drain
                    let spell = Drain::new();
                    if let Some(future) = self.attempt_spell(&spell) {
                        ret.push(future)
                    }
                    //Magic Missile
                    let spell = MagicMissile::new();
                    if let Some(future) = self.attempt_spell(&spell) {
                        ret.push(future)
                    }
                    //Poison
                    let spell = Poison::new();
                    if let Some(future) = self.attempt_spell(&spell) {
                        ret.push(future)
                    }
                    //Recharge
                    let spell = Recharge::new();
                    if let Some(future) = self.attempt_spell(&spell) {
                        ret.push(future)
                    }
                    //Shield
                    let spell = Shield::new();
                    if let Some(future) = self.attempt_spell(&spell) {
                        ret.push(future)
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
}
