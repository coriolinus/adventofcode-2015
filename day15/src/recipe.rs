
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::Iter;

use super::ingredient::Ingredient;

const TOTAL_INGREDIENTS: u16 = 100;

#[derive(Clone, PartialEq, Eq)]
pub struct Recipe {
    ingredients: HashMap<Ingredient, u16>,
}

impl Recipe {
    pub fn new(ings: Vec<Ingredient>) -> Recipe {
        let default_qty = TOTAL_INGREDIENTS / ings.len() as u16;
        let mut used = 0;

        let mut ret = HashMap::new();

        for ing in ings {
            let qty = if default_qty <= (TOTAL_INGREDIENTS - used) {
                default_qty
            } else {
                TOTAL_INGREDIENTS - used
            };

            ret.insert(ing, qty);
            used += qty;
        }

        Recipe { ingredients: ret }
    }

    pub fn goodness(&self) -> u32 {
        let mut capacity: i32 = 0;
        let mut durability: i32 = 0;
        let mut flavor: i32 = 0;
        let mut texture: i32 = 0;

        for (ing, qty) in &self.ingredients {
            capacity += *qty as i32 * ing.capacity;
            durability += *qty as i32 * ing.durability;
            flavor += *qty as i32 * ing.flavor;
            texture += *qty as i32 * ing.texture;
        }

        for quality in &mut [capacity, durability, flavor, texture] {
            if *quality < 0 {
                *quality = 0;
            }
        }

        (capacity * durability * flavor * texture) as u32
    }

    pub fn neighbors(&self) -> Neighbors {
        Neighbors::new(self)
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
}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.ingredients.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

pub struct Neighbors<'a> {
    recipe: &'a Recipe,
    incr_it: Iter<'a, Ingredient, u16>,
    decr_it: Iter<'a, Ingredient, u16>,
    incr_t: Option<(Ingredient, u16)>,
}

impl<'a> Neighbors<'a> {
    fn new(recipe: &Recipe) -> Neighbors {
        let mut n = Neighbors {
            recipe: recipe,
            incr_it: recipe.ingredients.iter(),
            decr_it: recipe.ingredients.iter(),
            incr_t: None,
        };
        n.increment_incr();
        n
    }

    fn increment_incr(&mut self) {
        self.incr_t = match self.incr_it.next() {
            Some((ing, iq)) => Some((ing.clone(), iq + 1)),
            _ => None,
        };
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item =  Recipe;

    fn next(&mut self) -> Option<Recipe> {
        if self.incr_t.is_none() {
            return None;
        }
        let (incr, inc_q) = self.incr_t.to_owned().unwrap();

        // ensure we're not trying to increase any quantity over the max
        if inc_q > TOTAL_INGREDIENTS {
            self.increment_incr();
            return self.next();
        }

        // get the ingredient to decrease
        let decr_t = self.decr_it.next();

        // reset and get the next increasing ingredient if we're out of ones to decrease
        if decr_t == None {
            self.increment_incr();
            return self.next();
        }

        // here, incr and decr are not None
        let (decr, &dec_q) = decr_t.unwrap();

        // continue iteration if we find the same ingredient we're increasing
        if *decr == incr {
            return self.next();
        }

        // ensure we're not trying to reduce any quantity below zero
        if dec_q >= 1 {
            // actually decrement the quantity of dec_q
            let dec_q = dec_q - 1;

            // finally construct a new Recipe
            let mut recipe = self.recipe.clone();
            recipe.ingredients.insert(incr.clone(), inc_q);
            recipe.ingredients.insert(decr.clone(), dec_q);
            assert_eq!(recipe.ingredients.values().fold(0, |acc, v| acc + v),
                       TOTAL_INGREDIENTS);
            Some(recipe)
        } else {
            // oops, just get the next thing
            self.next()
        }
    }
}
