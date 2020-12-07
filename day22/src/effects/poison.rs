use super::super::Character;

use super::EffectImpl;
use super::Effects;
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
        boss.hp = if boss.hp >= 3 { boss.hp - 3 } else { 0 };
    }

    fn per_turn_str(&self) -> String {
        let mut ret = format!("Poison deals 3 damage; its timer is now {}\n", self.ei.ttl);
        if self.ei.ttl == 0 {
            ret.push_str("Poison wears off.\n");
        }
        ret
    }

    fn to_impl(&self) -> EffectImpl {
        self.ei.clone()
    }
}
