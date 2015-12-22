use super::Character;

pub mod magic_missile;
pub mod drain;
pub mod shield;
pub mod poison;
pub mod recharge;

#[derive(PartialEq, Eq, Clone)]
pub enum Effects {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(PartialEq, Eq, Clone)]
pub struct EffectImpl {
    etype: Effects,
    name: String,
    mana_cost: u16,
    ttl: u8,
}


pub trait Magic {
    /// Represent this effect by its storable, clonable implementation.
    fn to_impl(&self) -> EffectImpl;

    /// This happens immediately when the player casts the spell.
    ///
    /// Responsible for consuming the requisite amount of mana, etc.
    fn on_cast(&self, player: &mut Character, boss: &mut Character);

    /// This happens per turn, beginning the turn after the player casts the spell.
    ///
    /// Responsible for decreasing its own time to live, etc.
    fn per_turn(&mut self, _: &mut Character, _: &mut Character) {}
}
