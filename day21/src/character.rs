use crate::loadout::Loadout;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CharacterType {
    Player,
    Boss,
}

impl Default for CharacterType {
    fn default() -> Self {
        CharacterType::Boss
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
#[display("Hit Points: {hp}\nDamage: {damage}\nArmor: {armor}\n")]
pub struct Character {
    #[from_str(default)]
    pub(crate) ctype: CharacterType,
    pub(crate) hp: u32,
    pub(crate) damage: u32,
    pub(crate) armor: u32,
}

impl From<&Loadout> for Character {
    fn from(loadout: &Loadout) -> Self {
        Character {
            ctype: CharacterType::Player,
            hp: 100,
            damage: loadout.damage(),
            armor: loadout.armor(),
        }
    }
}
