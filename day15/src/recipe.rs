use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

use crate::Ingredient;

pub(crate) const TOTAL_INGREDIENTS: u8 = 100;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Recipe {
    ingredients: Vec<Ingredient>,
    quantities: Vec<u8>,
}

impl FromIterator<Ingredient> for Recipe {
    fn from_iter<T: IntoIterator<Item = Ingredient>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (low_bound, _) = iter.size_hint();

        let mut ingredients = Vec::with_capacity(low_bound);
        ingredients.extend(iter);
        ingredients.sort_unstable_by_key(|ingredient| ingredient.name.clone());

        // return early in this case to avoid divide-by-0 error
        if ingredients.is_empty() {
            return Recipe {
                ingredients,
                quantities: Vec::new(),
            };
        }

        let per_ingredient = (TOTAL_INGREDIENTS as usize / ingredients.len()) as u8;
        let mut quantities = vec![per_ingredient; ingredients.len()];

        // add any excess to the first one to ensure that our total matches the real total
        quantities[0] += TOTAL_INGREDIENTS - quantities.iter().sum::<u8>();
        debug_assert_eq!(quantities.iter().sum::<u8>(), TOTAL_INGREDIENTS);

        Recipe {
            ingredients,
            quantities,
        }
    }
}

impl Recipe {
    fn with_quantities(&self, quantities: Vec<u8>) -> Recipe {
        Recipe {
            ingredients: self.ingredients.clone(),
            quantities,
        }
    }

    pub fn quantity_of(&self, name: &str) -> Option<u8> {
        self.ingredients
            .binary_search_by_key(&name, |ingredient| &ingredient.name)
            .ok()
            .map(|idx| self.quantities[idx])
    }

    pub fn goodness(&self) -> i32 {
        self.goodness_with(&self.quantities)
    }

    fn goodness_with(&self, quantities: &[u8]) -> i32 {
        let mut capacity: i32 = 0;
        let mut durability: i32 = 0;
        let mut flavor: i32 = 0;
        let mut texture: i32 = 0;

        for (ingredient, qty) in self.ingredients.iter().zip(quantities) {
            capacity += *qty as i32 * ingredient.capacity;
            durability += *qty as i32 * ingredient.durability;
            flavor += *qty as i32 * ingredient.flavor;
            texture += *qty as i32 * ingredient.texture;
        }

        for quality in &[capacity, durability, flavor, texture] {
            if *quality < 0 {
                return 0;
            }
        }

        capacity * durability * flavor * texture
    }

    pub fn climb_goodness(&self) -> Recipe {
        if self.ingredients.len() < 2 {
            // no neighbors can exist
            return self.clone();
        }
        // for 2 or more ingredients, at least one neighbor must exist

        let mut best_recipe = self.quantities.clone();
        let mut prev_best_goodness = -1; // goodness function never returns below 0
        let mut best_goodness = self.goodness();

        // Hill climb. If the best goodness stops increasing, then we've found
        // at least a local maximum, and we can stop.
        while best_goodness > prev_best_goodness {
            prev_best_goodness = best_goodness;
            let mut new_best_recipe = None;
            for quantities in neighbors_of(&best_recipe) {
                if self.goodness_with(&quantities) > best_goodness {
                    best_goodness = self.goodness_with(&quantities);
                    new_best_recipe = Some(quantities);
                }
            }
            if let Some(new_best_recipe) = new_best_recipe {
                best_recipe = new_best_recipe;
            }
        }

        self.with_quantities(best_recipe)
    }

    pub fn calories(&self) -> i32 {
        self.calories_with(&self.quantities)
    }

    fn calories_with(&self, quantities: &[u8]) -> i32 {
        self.ingredients
            .iter()
            .zip(quantities)
            .map(|(ingredient, &qty)| ingredient.calories * qty as i32)
            .sum()
    }

    /// Exhaustively check all possible recipes, returning the best of those (by goodness)
    /// which meets the calories constraint.
    pub fn exhaust_goodness_constrained(&self, calories: i32) -> Option<Recipe> {
        let mut best_recipe = None;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.quantities.clone());

        while let Some(quantities) = queue.pop_front() {
            if !visited.insert(quantities.clone()) {
                // insert returns false if the value was already present in the set
                continue;
            }

            // add future work
            queue.extend(neighbors_of(&quantities).filter(|quantity| !visited.contains(quantity)));

            // check this recipe
            if self.calories_with(&quantities) == calories {
                best_recipe = match best_recipe {
                    None => Some(quantities),
                    Some(cur_best_recipe) => Some(
                        if self.goodness_with(&quantities) > self.goodness_with(&cur_best_recipe) {
                            quantities
                        } else {
                            cur_best_recipe
                        },
                    ),
                };
            }
        }

        best_recipe.map(|quantities| self.with_quantities(quantities))
    }
}

/// Return an iterator of neighbors of the given quantity set.
///
/// Each returned neighbor is a copy of the original quantities with one of its
/// elements increased by one, and a different one decreased by one.
fn neighbors_of(quantities: &[u8]) -> impl '_ + Iterator<Item = Vec<u8>> {
    (0..quantities.len())
        .cartesian_product(0..quantities.len())
        .filter(move |(decr, incr)| decr != incr && quantities[*decr] > 0)
        .map(move |(decr, incr)| {
            let mut quantities = quantities.to_vec();
            quantities[decr] -= 1;
            quantities[incr] += 1;
            quantities
        })
}

#[cfg(test)]
mod tests {
    use crate::{recipe::Recipe, Ingredient};

    const EXAMPLE: &str = "
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    ";

    fn example() -> impl Iterator<Item = Ingredient> {
        EXAMPLE.trim().split('\n').map(|line| line.parse().unwrap())
    }

    #[test]
    fn test_from_str() {
        assert_eq!(example().count(), 2);
    }

    #[test]
    fn test_hill_climb_example() {
        let recipe = example().collect::<Recipe>().climb_goodness();
        dbg!(&recipe);

        assert_eq!(recipe.goodness(), 62842880);
        assert_eq!(recipe.quantity_of("Butterscotch").unwrap(), 44);
        assert_eq!(recipe.quantity_of("Cinnamon").unwrap(), 56);
    }

    #[test]
    fn test_exhaust_example_constrained() {
        let recipe = example()
            .collect::<Recipe>()
            .exhaust_goodness_constrained(500)
            .unwrap();
        dbg!(&recipe);

        assert_eq!(recipe.goodness(), 57600000);
        assert_eq!(recipe.quantity_of("Butterscotch").unwrap(), 40);
        assert_eq!(recipe.quantity_of("Cinnamon").unwrap(), 60);
    }
}
