use super::super::Character;

use super::Effects;
use super::EffectImpl;
use super::Magic;

pub struct Poison {
    ei: EffectImpl,
}

impl Poison {
    pub fn new() -> Poison {
        Poison {
            ei: EffectImpl {
                etype: Effects::Poison,
                name: "Poison".to_string(),
                mana_cost: 173,
                ttl: 6,
            },
        }
    }

    pub fn from_ei(ei: EffectImpl) -> Poison {
        Poison { ei: ei }
    }
}

impl Magic for Poison {
    fn on_cast(&self, player: &mut Character, _: &mut Character) {
        player.mana -= self.ei.mana_cost;
    }

    fn per_turn(&mut self, _: &mut Character, boss: &mut Character) {
        self.ei.ttl -= 1;
        boss.hp = if boss.hp >= 3 {
            boss.hp - 3
        } else {
            0
        };
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
