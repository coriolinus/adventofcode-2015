use super::super::Character;

use super::EffectImpl;
use super::Effects;
use super::Magic;

pub struct Shield {
    ei: EffectImpl,
}

impl Default for Shield {
    fn default() -> Self {
        Shield::new()
    }
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
        Shield { ei }
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

    fn per_turn_str(&self) -> String {
        let mut ret = format!("Shield's timer is now {}\n", self.ei.ttl);
        if self.ei.ttl == 0 {
            ret.push_str("Shield wears off, decreasing armor by 7\n");
        }
        ret
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
