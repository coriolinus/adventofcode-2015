use super::super::Character;

use super::Effects;
use super::EffectImpl;
use super::Magic;

pub struct MagicMissile {
    ei: EffectImpl,
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
}

impl Magic for MagicMissile {
    fn on_cast(&self, player: &mut Character, boss: &mut Character) {
        player.mana -= self.ei.mana_cost;
        boss.hp = if boss.hp >= 4 {
            boss.hp - 4
        } else {
            0
        };
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
