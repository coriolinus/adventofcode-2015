use super::super::Character;

use super::Effects;
use super::EffectImpl;
use super::Magic;

pub struct Recharge {
    ei: EffectImpl,
}

impl Recharge {
    pub fn new() -> Recharge {
        Recharge {
            ei: EffectImpl {
                etype: Effects::Recharge,
                name: "Recharge".to_string(),
                mana_cost: 229,
                ttl: 5,
            },
        }
    }

    pub fn from_ei(ei: EffectImpl) -> Recharge {
        Recharge { ei: ei }
    }
}

impl Magic for Recharge {
    fn on_cast(&self, player: &mut Character, _: &mut Character) {
        player.mana -= self.ei.mana_cost;
    }

    fn per_turn(&mut self, player: &mut Character, _: &mut Character) {
        self.ei.ttl -= 1;
        player.mana += 101;
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
