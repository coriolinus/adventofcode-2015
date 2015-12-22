use super::super::Character;

use super::Effects;
use super::EffectImpl;
use super::Magic;

pub struct Shield {
    ei: EffectImpl,
}

impl Shield {
    pub fn new() -> Shield {
        Shield {
            ei: EffectImpl {
                etype: Effects::Shield,
                name: "Shield".to_string(),
                mana_cost: 113,
                ttl: 6,
            },
        }
    }

    pub fn from_ei(ei: EffectImpl) -> Shield {
        Shield { ei: ei }
    }
}

impl Magic for Shield {
    fn on_cast(&self, player: &mut Character, _: &mut Character) {
        player.mana -= self.ei.mana_cost;
        player.armor += 7;
    }

    fn per_turn(&mut self, player: &mut Character, _: &mut Character) {
        self.ei.ttl -= 1;
        if self.ei.ttl == 0 {
            player.armor -= 7;
        }
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
