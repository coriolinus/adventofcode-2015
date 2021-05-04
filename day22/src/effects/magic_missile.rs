use super::super::Character;

use super::EffectImpl;
use super::Effects;
use super::Magic;

pub struct MagicMissile {
    ei: EffectImpl,
}

impl Default for MagicMissile {
    fn default() -> Self {
        MagicMissile::new()
    }
}

impl MagicMissile {
    pub fn new() -> MagicMissile {
        MagicMissile {
            ei: EffectImpl {
                etype: Effects::MagicMissile,
                name: "Magic Missile".to_string(),
                mana_cost: 53,
                ttl: 0,
            },
        }
    }

    pub fn from_ei(ei: EffectImpl) -> MagicMissile {
        MagicMissile { ei }
    }
}

impl Magic for MagicMissile {
    fn on_cast(&self, player: &mut Character, boss: &mut Character) {
        player.mana -= self.ei.mana_cost;
        boss.hp = if boss.hp >= 4 { boss.hp - 4 } else { 0 };
    }

    fn on_cast_str(&self) -> String {
        "Player casts Magic Missile, dealing 4 damage\n".to_string()
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
