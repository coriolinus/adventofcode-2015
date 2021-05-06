use crate::items::{Item, ItemType};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Loadout {
    pub(crate) weapon: Item,
    pub(crate) armor: Option<Item>,
    pub(crate) ring_l: Option<Item>,
    pub(crate) ring_r: Option<Item>,
}

impl Loadout {
    pub fn new(
        weapon: Item,
        armor: Option<Item>,
        ring_l: Option<Item>,
        ring_r: Option<Item>,
    ) -> Option<Loadout> {
        let expect_type = |item: Option<Item>, item_type: ItemType| match item {
            None => Some(()),
            Some(item) => (item.itype == item_type).then(|| ()),
        };

        expect_type(Some(weapon), ItemType::Weapon)?;
        expect_type(armor, ItemType::Armor)?;
        expect_type(ring_l, ItemType::Ring)?;
        expect_type(ring_r, ItemType::Ring)?;

        Some(Loadout {
            weapon,
            armor,
            ring_l,
            ring_r,
        })
    }

    fn equipped(&self) -> impl Iterator<Item = Option<Item>> {
        std::array::IntoIter::new([Some(self.weapon), self.armor, self.ring_l, self.ring_r])
    }

    fn equipped_sum_by(&self, selector: impl Fn(Item) -> u32) -> u32 {
        self.equipped()
            .filter_map(|maybe_item| maybe_item.map(&selector))
            .sum()
    }

    pub fn cost(&self) -> u32 {
        self.equipped_sum_by(|item| item.cost)
    }

    pub fn damage(&self) -> u32 {
        self.equipped_sum_by(|item| item.damage)
    }

    pub fn armor(&self) -> u32 {
        self.equipped_sum_by(|item| item.armor)
    }
}
