use super::super::Character;

use super::EffectImpl;
use super::Effects;
use super::Magic;

pub struct Recharge {
    ei: EffectImpl,
}

impl Default for Recharge {
    fn default() -> Self {
        Recharge::new()
    }
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
        Recharge { ei }
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

    fn per_turn_str(&self) -> String {
        let mut ret = format!(
            "Recharge provides 101 mana; its timer is now {}\n",
            self.ei.ttl
        );
        if self.ei.ttl == 0 {
            ret.push_str("Recharge wears off.\n");
        }
        ret
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
