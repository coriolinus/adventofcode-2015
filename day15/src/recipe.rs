use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use crate::{neighbors::Neighbors, Ingredient};

pub(crate) const TOTAL_INGREDIENTS: i32 = 100;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Recipe {
    pub ingredients: HashMap<Ingredient, i32>,
}

impl FromIterator<Ingredient> for Recipe {
    fn from_iter<T: IntoIterator<Item = Ingredient>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (low_bound, _) = iter.size_hint();
        let mut ingredients = HashMap::with_capacity(low_bound);

        for ingredient in iter {
            ingredients.insert(ingredient, 0);
        }

        if ingredients.is_empty() {
            return Recipe { ingredients };
        }

        let per_ingredient = TOTAL_INGREDIENTS / ingredients.len() as i32;
        for value in ingredients.values_mut() {
            *value = per_ingredient;
        }

        let shortfall = TOTAL_INGREDIENTS - ingredients.values().sum::<i32>();
        if shortfall > 0 {
            *ingredients
                .values_mut()
                .next()
                .expect("ingredients is not empty; qed") += shortfall;
        }

        assert_eq!(ingredients.values().sum::<i32>(), TOTAL_INGREDIENTS);

        Recipe { ingredients }
    }
}

impl Recipe {
    pub fn goodness(&self) -> i32 {
        let mut capacity: i32 = 0;
        let mut durability: i32 = 0;
        let mut flavor: i32 = 0;
        let mut texture: i32 = 0;

        for (ing, qty) in &self.ingredients {
            capacity += *qty * ing.capacity;
            durability += *qty * ing.durability;
            flavor += *qty * ing.flavor;
            texture += *qty * ing.texture;
        }

        for quality in &[capacity, durability, flavor, texture] {
            if *quality < 0 {
                return 0;
            }
        }

        capacity * durability * flavor * texture
    }

    pub fn neighbors(&self) -> Neighbors {
        self.into()
    }

    pub fn climb_goodness(&self) -> Recipe {
        if self.ingredients.len() < 2 {
            // no neighbors can exist
            return self.clone();
        }
        // for 2 or more ingredients, at least one neighbor must exist

        let mut best_recipe = self.to_owned();
        let mut prev_best_recipe = self.to_owned();
        let mut best_goodness = self.goodness();

        loop {
            for n in prev_best_recipe.neighbors() {
                // println!(" - {}", n);
                if n.goodness() > best_goodness {
                    best_recipe = n.to_owned();
                    best_goodness = n.goodness();
                }
            }
            if best_recipe == prev_best_recipe {
                // we've found at least a local maximum
                break;
            } else {
                prev_best_recipe = best_recipe.to_owned();
            }
        }
        best_recipe
    }

    pub fn calories(&self) -> i32 {
        self.ingredients
            .iter()
            .map(|(ing, &qty)| ing.calories * qty as i32)
            .fold(0, |acc, val| acc + val)
    }

    pub fn exhaust_goodness_constrained(&self, calories: i32) -> Option<Recipe> {
        let mut best_constrained_recipe = if self.calories() == calories {
            Some(self.to_owned())
        } else {
            None
        };
        if self.ingredients.len() < 2 {
            // no neighbors can exist
            return best_constrained_recipe;
        }
        // for 2 or more ingredients, at least one neighbor must exist

        let mut visited = HashSet::new();
        let mut future = Vec::new();
        let mut to_examine = self.neighbors().collect::<Vec<_>>();

        loop {
            for recipe in to_examine {
                // skip work if we can
                if visited.contains(&recipe) {
                    continue;
                }
                visited.insert(recipe.to_owned());

                // add future work
                future.extend(recipe.neighbors());

                // check this recipe
                if recipe.calories() == calories
                    && (best_constrained_recipe.is_none()
                        || recipe.goodness() > best_constrained_recipe.clone().unwrap().goodness())
                {
                    best_constrained_recipe = Some(recipe.clone());
                }
            }

            // reset to_examine to a list of items not already seen
            to_examine = future
                .iter()
                .filter(|r| !visited.contains(r))
                .cloned()
                .collect::<Vec<_>>();
            if to_examine.len() == 0 {
                break;
            }
            future = Vec::new();
        }

        best_constrained_recipe
    }
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.ingredients.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}
