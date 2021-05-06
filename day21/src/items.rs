#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Item {
    pub(crate) name: &'static str,
    pub(crate) itype: ItemType,
    pub(crate) cost: u32,
    pub(crate) damage: u32,
    pub(crate) armor: u32,
}

impl Item {
    fn bare(itype: ItemType) -> Item {
        Item {
            name: "",
            itype,
            cost: 0,
            damage: 0,
            armor: 0,
        }
    }

    fn weapon(name: &'static str, cost: u32, damage: u32) -> Item {
        Item {
            name,
            cost,
            damage,
            ..Item::bare(ItemType::Weapon)
        }
    }

    fn armor(name: &'static str, cost: u32, armor: u32) -> Item {
        Item {
            name,
            cost,
            armor,
            ..Item::bare(ItemType::Armor)
        }
    }

    fn ring(name: &'static str, cost: u32, damage: u32, armor: u32) -> Item {
        Item {
            name,
            cost,
            damage,
            armor,
            ..Item::bare(ItemType::Ring)
        }
    }
}

pub fn item_shop() -> Vec<Item> {
    vec![
        // weapons
        Item::weapon("Dagger", 8, 4),
        Item::weapon("Shortsword", 10, 5),
        Item::weapon("Warhammer", 25, 6),
        Item::weapon("Longsword", 40, 7),
        Item::weapon("Greataxe", 74, 8),
        // armor
        Item::armor("Leather", 13, 1),
        Item::armor("Chainmail", 31, 2),
        Item::armor("Splintmail", 53, 3),
        Item::armor("Bandedmail", 75, 4),
        Item::armor("Platemail", 102, 5),
        // rings
        Item::ring("Defense +1", 20, 0, 1),
        Item::ring("Damage +1", 25, 1, 0),
        Item::ring("Defense +2", 40, 0, 2),
        Item::ring("Damage +2", 50, 2, 0),
        Item::ring("Defense +3", 80, 0, 3),
        Item::ring("Damage +3", 100, 3, 0),
    ]
}
