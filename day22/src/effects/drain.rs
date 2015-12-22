use super::super::Character;

use super::Effects;
use super::EffectImpl;
use super::Magic;

pub struct Drain {
    ei: EffectImpl,
}

impl Drain {
    pub fn new() -> Drain {
        Drain {
            ei: EffectImpl {
                etype: Effects::Drain,
                name: "Drain".to_string(),
                mana_cost: 73,
                ttl: 0,
            },
        }
    }
}

impl Magic for Drain {
    fn on_cast(&self, player: &mut Character, boss: &mut Character) {
        player.mana -= self.ei.mana_cost;
        player.hp += 2;
        boss.hp = if boss.hp >= 2 {
            boss.hp - 2
        } else {
            0
        };
    }
}
